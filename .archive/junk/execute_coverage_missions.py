#!/usr/bin/env python3
import subprocess
import os
import time

missions = [
    "missions/coverage/coverage_mission_001_src_core_chain.yaml",
    "missions/coverage/coverage_mission_002_src_llm_azure_openai_provider.yaml",
    "missions/coverage/coverage_mission_003_src_build_dashboard.yaml",
    "missions/coverage/coverage_mission_004_src_validation_mod.yaml",
    "missions/coverage/coverage_mission_005_src_core_features.yaml",
    "missions/coverage/coverage_mission_006_src_server_mod.yaml",
    "missions/coverage/coverage_mission_007_src_core_audit.yaml",
    "missions/coverage/coverage_mission_008_src_security_access_control.yaml",
    "missions/coverage/coverage_mission_009_src_core_plugin.yaml",
    "missions/coverage/coverage_mission_010_src_core_document_loaders.yaml",
    "missions/coverage/coverage_mission_011_src_security_encryption.yaml",
    "missions/coverage/coverage_mission_012_src_telemetry_mod.yaml",
    "missions/coverage/coverage_mission_013_src_tools_mod.yaml",
    "missions/coverage/coverage_mission_014_src_core_tools.yaml",
    "missions/coverage/coverage_mission_015_src_security_compliance.yaml",
    "missions/coverage/coverage_mission_016_src_llm_google_gemini_provider.yaml",
    "missions/coverage/coverage_mission_017_src_llm_test_connectivity.yaml",
    "missions/coverage/coverage_mission_018_src_security_threat_detection.yaml",
    "missions/coverage/coverage_mission_019_src_security_audit.yaml",
    "missions/coverage/coverage_mission_020_src_registry_mod.yaml",
    "missions/coverage/coverage_mission_021_src_llm_aws_bedrock_provider.yaml",
    "missions/coverage/coverage_mission_022_src_engine_mod.yaml",
    "missions/coverage/coverage_mission_023_src_core_memory.yaml",
    "missions/coverage/coverage_mission_024_src_compliance_mod.yaml",
    "missions/coverage/coverage_mission_025_src_llm_shimmy_provider.yaml",
    "missions/coverage/coverage_mission_026_src_smt_compliance_engine.yaml",
    "missions/coverage/coverage_mission_027_src_compliance_sdk.yaml",
    "missions/coverage/coverage_mission_028_src_core_mod.yaml",
    "missions/coverage/coverage_mission_029_src_core_agent.yaml",
    "missions/coverage/coverage_mission_030_src_sandbox_mod.yaml",
    "missions/coverage/coverage_mission_031_src_core_python_interpreter.yaml",
    "missions/coverage/coverage_mission_032_src_llm_mod.yaml",
    "missions/coverage/coverage_mission_033_src_core_error_formatting.yaml",
    "missions/coverage/coverage_mission_034_src_core_web_search_tools.yaml",
    "missions/coverage/coverage_mission_035_src_security_auth.yaml",
    "missions/coverage/coverage_mission_036_src_smt_solver.yaml",
    "missions/coverage/coverage_mission_037_src_core_llm.yaml",
    "missions/coverage/coverage_mission_038_src_invariant_ppt.yaml",
    "missions/coverage/coverage_mission_039_src_core_github_toolkit.yaml",
    "missions/coverage/coverage_mission_040_src_core_pinecone_vector_store.yaml",
    "missions/coverage/coverage_mission_041_src_core_chroma_vector_store.yaml",
    "missions/coverage/coverage_mission_042_src_rag_mod.yaml",
    "missions/coverage/coverage_mission_043_src_security_mod.yaml",
    "missions/coverage/coverage_mission_044_src_art_mod.yaml",
    "missions/coverage/coverage_mission_045_src_art_performance.yaml",
    "missions/coverage/coverage_mission_046_src_art_trajectory.yaml",
    "missions/coverage/coverage_mission_047_src_cli_handlers_enterprise.yaml",
    "missions/coverage/coverage_mission_048_src_cli_handlers_mod.yaml",
    "missions/coverage/coverage_mission_049_src_cli_interactive.yaml",
    "missions/coverage/coverage_mission_050_src_cli_mod.yaml",
    "missions/coverage/coverage_mission_051_src_concurrency.yaml",
    "missions/coverage/coverage_mission_052_src_core_executor.yaml",
    "missions/coverage/coverage_mission_053_src_core_mission.yaml",
    "missions/coverage/coverage_mission_054_src_engine_chain_executor.yaml",
    "missions/coverage/coverage_mission_055_src_infrastructure_schema.yaml",
    "missions/coverage/coverage_mission_056_src_smt_constraints.yaml",
    "missions/coverage/coverage_mission_057_src_smt_contextlite_bridge.yaml",
    "missions/coverage/coverage_mission_058_src_smt_mod.yaml",
    "missions/coverage/coverage_mission_059_src_smt_verification.yaml",
    "missions/coverage/coverage_mission_060_src_testing_invariants.yaml",
]

def execute_mission(mission_path):
    """Execute a single coverage mission"""
    print(f"Executing: {mission_path}")
    result = subprocess.run([
        "cargo", "run", "--bin", "rustchain", "--features", "llm", 
        "--", "run", mission_path
    ], capture_output=True, text=True)
    
    return result.returncode == 0, result.stdout, result.stderr

def main():
    successful = 0
    failed = 0
    
    for mission in missions:
        if not os.path.exists(mission):
            print(f"SKIP: {mission} not found")
            continue
            
        success, stdout, stderr = execute_mission(mission)
        
        if success:
            print(f"SUCCESS: {mission}")
            successful += 1
        else:
            print(f"FAILED: {mission}")
            print(f"Error: {stderr}")
            failed += 1
        
        # Brief pause between missions
        time.sleep(5)
    
    print(f"\nExecution complete: {successful} successful, {failed} failed")
    
    # Generate final coverage report
    subprocess.run(["python", "create_enhanced_coverage_registry.py"])

if __name__ == "__main__":
    main()
