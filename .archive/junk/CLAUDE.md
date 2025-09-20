# CRITICAL CODING RULES

## PROFESSIONAL COMMUNICATION - NON-NEGOTIABLE
- Maintain professional language and tone always
- Focus on technical accuracy and clear problem-solving

## NO EMOJIS IN CODE - EVER
- ASCII ONLY in all code - no exceptions
- Use plain text: "INFO:", "ERROR:", "SUCCESS:" instead of emojis
- Unicode/emoji characters cause encoding errors across different systems

## CLAUDE CODE FILE OPERATIONS - MANDATORY
- **MUST read a file before attempting to write/edit it**
- Always use Read tool first, then Edit/Write - NO EXCEPTIONS

## PUNCH Quick Commands
**Instant Analysis:** `punch go .` | `punch quality .` | `punch stats`

## DIRECTORY CONTEXT PATTERN
**CRITICAL**: Directory references (e.g., "command-center", "contextlite", "shimmy") refer to `C:\Users\micha\repos\`

## GITHUB SPEC KIT - REVOLUTIONARY SDD FRAMEWORK

**What it is**: Spec-Driven Development framework that makes specifications executable
- **Workflow**: `/specify` → `/plan` → `/tasks` → implement  
- **AI Integration**: Native support for Claude Code, Copilot, Gemini, Cursor
- **Constitutional Development**: Enforced architectural principles

**CRITICAL ENCODING FIX**:
```bash
PYTHONIOENCODING=utf-8 specify [command]
```

**Setup for RustChain**:
```bash
cd C:\Users\micha\repos\rustchain-community
uvx --from git+https://github.com/github/spec-kit.git specify init --here --ai claude
```

**Next Steps**:
1. Initialize Spec Kit immediately  
2. Create retroactive specifications for Mission Engine, CLI System, Tool Framework
3. Use for all future RustChain feature development

## PRODUCTION STATUS
- ✅ **97%+ test success rate** - All critical systems operational
- ✅ **Mission execution working** - DAG executor with 12 step types
- ✅ **Enterprise features** - Audit trails, policy engine, safety validation
- ✅ **Clean compilation** - Zero errors, minimal warnings

## GIT WORKFLOW
- **Commit Format**: `type(scope): description` with conventional commits
- **Branch Strategy**: `main` (production), `feature/description` (development)
- **Test Before Commit**: Ensure `cargo test` passes

## WEBSITE & WIKI MAINTENANCE

### CRITICAL PATHS
- **Website Repository**: `C:\Users\micha\repos\rustchain-community\website\` (deploys to rust-chain-forge)
- **Wiki Repository**: `C:\Users\micha\repos\rustchain-community.wiki\` (GitHub wiki)
- **Main Documentation**: `C:\Users\micha\repos\rustchain-community\docs\`

### AUTOMATIC WIKI SYNC
The website **automatically syncs** with the GitHub wiki every 5 minutes via `WikiService`:
- **Source**: `rustchain-community/rustchain-community.wiki` repository
- **Target**: Website wiki viewer (localhost:8087/#wiki)
- **Cache**: 5-minute refresh cycle
- **Fallback**: Static content if GitHub unavailable

### MAINTENANCE WORKFLOW

**To Update Website:**
1. Edit files in `C:\Users\micha\repos\rustchain-community\website\`
2. Test locally: `npm run dev` (localhost:8087)
3. Commit and push to `rust-chain-forge` main branch
4. Cloudflare auto-deploys within 2-3 minutes

**To Update Wiki:**
1. Edit markdown files in `C:\Users\micha\repos\rustchain-community.wiki\`
2. Commit and push to GitHub wiki repository
3. Website automatically syncs within 5 minutes (no rebuild needed)
4. Changes appear in website wiki viewer automatically

**Both Systems:**
- Use professional language (no emojis in code)
- Follow conventional commit format
- Test changes locally before pushing
- Wiki changes flow to website automatically
- Website changes require rebuild/deployment

### WIKI FRONTMATTER FORMAT
```yaml
---
title: "Page Title"
category: "quickstart|development|enterprise|overview"
tags: ["installation", "api", "security"]
lastUpdated: "2024-12-16"
---
```

## CURRENT PRIORITIES
1. **Initialize GitHub Spec Kit** for comprehensive specifications
2. **Fix any remaining test failures** for 100% success rate
3. **Create specifications** for Mission Engine, CLI System, Tool Framework
4. **Community-driven development** through structured specifications

