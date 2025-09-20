use std::collections::{HashMap, HashSet};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::Timelike;
use crate::security::SecurityContext;

/// Role-Based Access Control (RBAC) structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,
    pub inherits_from: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
    pub resource: String,
    pub action: String,
    pub conditions: Vec<AccessCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    TimeOfDay { start_hour: u8, end_hour: u8 },
    IpRange { cidr: String },
    TenantMatch { tenant_id: String },
    SecurityLevelMinimum { level: String },
    Custom { condition: String, value: String },
}

/// Authorization service trait
#[async_trait]
pub trait AuthorizationService: Send + Sync {
    async fn authorize(&self, context: &SecurityContext, resource: &str, action: &str) -> crate::core::error::Result<bool>;
    async fn get_user_permissions(&self, user_id: &str) -> crate::core::error::Result<Vec<String>>;
    async fn assign_role(&self, user_id: &str, role: &str) -> crate::core::error::Result<()>;
    async fn revoke_role(&self, user_id: &str, role: &str) -> crate::core::error::Result<()>;
    async fn create_role(&self, role: Role) -> crate::core::error::Result<()>;
    async fn delete_role(&self, role_name: &str) -> crate::core::error::Result<()>;
}

/// RBAC implementation of authorization service
pub struct RbacAuthorizationService {
    roles: HashMap<String, Role>,
    user_roles: HashMap<String, HashSet<String>>,
    permissions: HashMap<String, Permission>,
}

impl RbacAuthorizationService {
    pub fn new() -> Self {
        let mut service = Self {
            roles: HashMap::new(),
            user_roles: HashMap::new(),
            permissions: HashMap::new(),
        };
        
        // Initialize default roles and permissions
        service.initialize_default_rbac();
        
        service
    }
    
    fn initialize_default_rbac(&mut self) {
        // Define default permissions
        let permissions = vec![
            Permission {
                name: "read:missions".to_string(),
                resource: "missions".to_string(),
                action: "read".to_string(),
                conditions: vec![],
            },
            Permission {
                name: "write:missions".to_string(),
                resource: "missions".to_string(),
                action: "write".to_string(),
                conditions: vec![],
            },
            Permission {
                name: "delete:missions".to_string(),
                resource: "missions".to_string(),
                action: "delete".to_string(),
                conditions: vec![AccessCondition::SecurityLevelMinimum {
                    level: "confidential".to_string(),
                }],
            },
            Permission {
                name: "admin:system".to_string(),
                resource: "system".to_string(),
                action: "admin".to_string(),
                conditions: vec![
                    AccessCondition::TimeOfDay { start_hour: 8, end_hour: 18 },
                    AccessCondition::SecurityLevelMinimum {
                        level: "restricted".to_string(),
                    },
                ],
            },
            Permission {
                name: "read:audit".to_string(),
                resource: "audit".to_string(),
                action: "read".to_string(),
                conditions: vec![],
            },
            Permission {
                name: "write:policies".to_string(),
                resource: "policies".to_string(),
                action: "write".to_string(),
                conditions: vec![],
            },
        ];
        
        for permission in permissions {
            self.permissions.insert(permission.name.clone(), permission);
        }
        
        // Define default roles
        let roles = vec![
            Role {
                name: "viewer".to_string(),
                description: "Can view missions and basic system information".to_string(),
                permissions: vec!["read:missions".to_string()].into_iter().collect(),
                inherits_from: vec![],
            },
            Role {
                name: "user".to_string(),
                description: "Can create, read, and modify missions".to_string(),
                permissions: vec![
                    "read:missions".to_string(),
                    "write:missions".to_string(),
                ].into_iter().collect(),
                inherits_from: vec!["viewer".to_string()],
            },
            Role {
                name: "operator".to_string(),
                description: "Can manage missions and view audit logs".to_string(),
                permissions: vec![
                    "read:missions".to_string(),
                    "write:missions".to_string(),
                    "delete:missions".to_string(),
                    "read:audit".to_string(),
                ].into_iter().collect(),
                inherits_from: vec!["user".to_string()],
            },
            Role {
                name: "administrator".to_string(),
                description: "Full system access".to_string(),
                permissions: vec![
                    "read:missions".to_string(),
                    "write:missions".to_string(),
                    "delete:missions".to_string(),
                    "admin:system".to_string(),
                    "read:audit".to_string(),
                    "write:policies".to_string(),
                ].into_iter().collect(),
                inherits_from: vec!["operator".to_string()],
            },
        ];
        
        for role in roles {
            self.roles.insert(role.name.clone(), role);
        }
        
        // Assign default roles to demo users
        self.user_roles.insert("admin".to_string(), vec!["administrator".to_string()].into_iter().collect());
        self.user_roles.insert("user".to_string(), vec!["user".to_string()].into_iter().collect());
        self.user_roles.insert("api_client".to_string(), vec!["operator".to_string()].into_iter().collect());
    }
    
    fn resolve_user_permissions(&self, user_id: &str) -> HashSet<String> {
        let mut permissions = HashSet::new();
        
        if let Some(roles) = self.user_roles.get(user_id) {
            for role_name in roles {
                if let Some(role) = self.roles.get(role_name) {
                    // Add direct permissions
                    permissions.extend(role.permissions.iter().cloned());
                    
                    // Add inherited permissions
                    self.resolve_inherited_permissions(role, &mut permissions);
                }
            }
        }
        
        permissions
    }
    
    fn resolve_inherited_permissions(&self, role: &Role, permissions: &mut HashSet<String>) {
        for inherited_role_name in &role.inherits_from {
            if let Some(inherited_role) = self.roles.get(inherited_role_name) {
                permissions.extend(inherited_role.permissions.iter().cloned());
                self.resolve_inherited_permissions(inherited_role, permissions);
            }
        }
    }
    
    fn check_conditions(&self, conditions: &[AccessCondition], context: &SecurityContext) -> bool {
        for condition in conditions {
            match condition {
                AccessCondition::TimeOfDay { start_hour, end_hour } => {
                    let current_hour = chrono::Utc::now().time().hour() as u8;
                    if current_hour < *start_hour || current_hour > *end_hour {
                        return false;
                    }
                }
                AccessCondition::IpRange { cidr: _ } => {
                    // In a real implementation, this would check IP ranges
                    // For now, we'll always pass this check
                }
                AccessCondition::TenantMatch { tenant_id } => {
                    if context.tenant_id.as_ref() != Some(tenant_id) {
                        return false;
                    }
                }
                AccessCondition::SecurityLevelMinimum { level } => {
                    let required_level = match level.as_str() {
                        "public" => 0,
                        "internal" => 1,
                        "confidential" => 2,
                        "restricted" => 3,
                        "topsecret" => 4,
                        _ => 0,
                    };
                    
                    let context_level = match context.security_level {
                        crate::security::SecurityLevel::Public => 0,
                        crate::security::SecurityLevel::Internal => 1,
                        crate::security::SecurityLevel::Confidential => 2,
                        crate::security::SecurityLevel::Restricted => 3,
                        crate::security::SecurityLevel::TopSecret => 4,
                    };
                    
                    if context_level < required_level {
                        return false;
                    }
                }
                AccessCondition::Custom { condition: _, value: _ } => {
                    // In a real implementation, this would evaluate custom conditions
                    // For now, we'll always pass this check
                }
            }
        }
        
        true
    }
}

#[async_trait]
impl AuthorizationService for RbacAuthorizationService {
    async fn authorize(&self, context: &SecurityContext, resource: &str, action: &str) -> crate::core::error::Result<bool> {
        let user_id = context.user_id.as_deref().unwrap_or("anonymous");
        let user_permissions = self.resolve_user_permissions(user_id);
        
        // Check for direct permission match
        let permission_key = format!("{}:{}", action, resource);
        
        if user_permissions.contains(&permission_key) {
            // Check if permission has conditions
            if let Some(permission) = self.permissions.get(&permission_key) {
                return Ok(self.check_conditions(&permission.conditions, context));
            }
            return Ok(true);
        }
        
        // Check for wildcard permissions
        let wildcard_permission = format!("{}:*", action);
        if user_permissions.contains(&wildcard_permission) {
            return Ok(true);
        }
        
        let resource_wildcard = format!("*:{}", resource);
        if user_permissions.contains(&resource_wildcard) {
            return Ok(true);
        }
        
        // Check for admin permission
        if user_permissions.contains("admin:system") {
            if let Some(permission) = self.permissions.get("admin:system") {
                return Ok(self.check_conditions(&permission.conditions, context));
            }
            return Ok(true);
        }
        
        Ok(false)
    }
    
    async fn get_user_permissions(&self, user_id: &str) -> crate::core::error::Result<Vec<String>> {
        let permissions = self.resolve_user_permissions(user_id);
        Ok(permissions.into_iter().collect())
    }
    
    async fn assign_role(&self, _user_id: &str, _role: &str) -> crate::core::error::Result<()> {
        // In a real implementation, this would persist the role assignment
        Ok(())
    }
    
    async fn revoke_role(&self, _user_id: &str, _role: &str) -> crate::core::error::Result<()> {
        // In a real implementation, this would remove the role assignment
        Ok(())
    }
    
    async fn create_role(&self, _role: Role) -> crate::core::error::Result<()> {
        // In a real implementation, this would persist the new role
        Ok(())
    }
    
    async fn delete_role(&self, _role_name: &str) -> crate::core::error::Result<()> {
        // In a real implementation, this would delete the role
        Ok(())
    }
}

/// Attribute-Based Access Control (ABAC) service
pub struct AbacAuthorizationService {
    policies: Vec<AbacPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacPolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rules: Vec<AbacRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacRule {
    pub subject_attributes: HashMap<String, String>,
    pub resource_attributes: HashMap<String, String>,
    pub action_attributes: HashMap<String, String>,
    pub environment_attributes: HashMap<String, String>,
    pub effect: PermissionEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionEffect {
    Allow,
    Deny,
}

impl AbacAuthorizationService {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }
    
    pub fn add_policy(&mut self, policy: AbacPolicy) {
        self.policies.push(policy);
    }
    
    fn evaluate_policy(&self, context: &SecurityContext, _resource: &str, action: &str) -> bool {
        // In a real implementation, this would evaluate ABAC policies
        // For now, we'll implement a simple demo
        
        let user_id = context.user_id.as_deref().unwrap_or("anonymous");
        
        // Allow if user is admin
        if user_id == "admin" {
            return true;
        }
        
        // Allow read operations for authenticated users
        if action == "read" && user_id != "anonymous" {
            return true;
        }
        
        false
    }
}

#[async_trait]
impl AuthorizationService for AbacAuthorizationService {
    async fn authorize(&self, context: &SecurityContext, resource: &str, action: &str) -> crate::core::error::Result<bool> {
        Ok(self.evaluate_policy(context, resource, action))
    }
    
    async fn get_user_permissions(&self, _user_id: &str) -> crate::core::error::Result<Vec<String>> {
        // ABAC doesn't have static permissions - they're evaluated dynamically
        Ok(vec!["dynamic:permissions".to_string()])
    }
    
    async fn assign_role(&self, _user_id: &str, _role: &str) -> crate::core::error::Result<()> {
        Err(crate::core::error::RustChainError::Security("ABAC doesn't use roles".to_string()))
    }
    
    async fn revoke_role(&self, _user_id: &str, _role: &str) -> crate::core::error::Result<()> {
        Err(crate::core::error::RustChainError::Security("ABAC doesn't use roles".to_string()))
    }
    
    async fn create_role(&self, _role: Role) -> crate::core::error::Result<()> {
        Err(crate::core::error::RustChainError::Security("ABAC doesn't use roles".to_string()))
    }
    
    async fn delete_role(&self, _role_name: &str) -> crate::core::error::Result<()> {
        Err(crate::core::error::RustChainError::Security("ABAC doesn't use roles".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::SecurityLevel;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_rbac_authorization() {
        let authz_service = RbacAuthorizationService::new();
        
        let context = SecurityContext {
            session_id: Uuid::new_v4(),
            user_id: Some("admin".to_string()),
            tenant_id: Some("default".to_string()),
            permissions: vec!["admin:system".to_string()],
            security_level: SecurityLevel::Restricted,
            created_at: chrono::Utc::now(),
            expires_at: None,
        };
        
        let authorized = authz_service.authorize(&context, "missions", "read").await.unwrap();
        assert!(authorized);
        
        let authorized = authz_service.authorize(&context, "system", "admin").await.unwrap();
        
        // The admin:system permission has time-based restrictions (8-18), so the result 
        // depends on the current time. Let's check both possibilities.
        let current_hour = chrono::Utc::now().time().hour() as u8;
        let is_business_hours = current_hour >= 8 && current_hour <= 18;
        assert_eq!(authorized, is_business_hours, "Admin access should only work during business hours (8-18), current hour: {}", current_hour);
    }
    
    #[tokio::test]
    async fn test_rbac_user_permissions() {
        let authz_service = RbacAuthorizationService::new();
        
        let permissions = authz_service.get_user_permissions("user").await.unwrap();
        assert!(permissions.contains(&"read:missions".to_string()));
        assert!(permissions.contains(&"write:missions".to_string()));
    }
    
    #[tokio::test]
    async fn test_unauthorized_access() {
        let authz_service = RbacAuthorizationService::new();
        
        let context = SecurityContext {
            session_id: Uuid::new_v4(),
            user_id: Some("unknown_user".to_string()),
            tenant_id: Some("default".to_string()),
            permissions: vec![],
            security_level: SecurityLevel::Public,
            created_at: chrono::Utc::now(),
            expires_at: None,
        };
        
        let authorized = authz_service.authorize(&context, "missions", "delete").await.unwrap();
        assert!(!authorized);
    }
}