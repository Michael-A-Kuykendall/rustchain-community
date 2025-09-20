# RustChain Codebase Analysis Script
param(
    [string]$JsonPath = "C:\Users\micha\repos\rustchain-community\rustchain_analysis.json"
)

Write-Host "Loading RustChain analysis data..." -ForegroundColor Cyan

# Load JSON data
$json = Get-Content $JsonPath | ConvertFrom-Json
$components = $json.components

Write-Host "Total components found: $($components.Count)" -ForegroundColor Green

# Type mappings (based on observed patterns)
$typeMap = @{
    0 = "Function"
    1 = "Struct" 
    4 = "Module"
    6 = "Enum"
    7 = "Trait"
    9 = "Impl"
    10 = "Const"
    16 = "Type/Alias"
}

# Component type distribution
Write-Host "`n=== COMPONENT TYPE DISTRIBUTION ===" -ForegroundColor Yellow
$typeGroups = $components | Group-Object type | Sort-Object Count -Descending
foreach ($group in $typeGroups) {
    $typeName = if ($typeMap.ContainsKey([int]$group.Name)) { $typeMap[[int]$group.Name] } else { "Unknown($($group.Name))" }
    Write-Host "$typeName : $($group.Count)" -ForegroundColor White
}

# Visibility analysis
Write-Host "`n=== VISIBILITY ANALYSIS ===" -ForegroundColor Yellow
$publicCount = ($components | Where-Object { $_.tags -contains "public" }).Count
$privateCount = ($components | Where-Object { $_.tags -contains "private" }).Count
Write-Host "Public: $publicCount" -ForegroundColor Green
Write-Host "Private: $privateCount" -ForegroundColor White
Write-Host "Public API Surface: $([math]::Round($publicCount/($publicCount+$privateCount)*100, 2))%" -ForegroundColor Cyan

# Complexity analysis
Write-Host "`n=== COMPLEXITY ANALYSIS ===" -ForegroundColor Yellow
$complexities = $components | Where-Object { $_.complexity -ne $null } | ForEach-Object { [int]$_.complexity }
$avgComplexity = ($complexities | Measure-Object -Average).Average
$maxComplexity = ($complexities | Measure-Object -Maximum).Maximum
Write-Host "Average complexity: $([math]::Round($avgComplexity, 2))" -ForegroundColor White
Write-Host "Maximum complexity: $maxComplexity" -ForegroundColor Red

# Find most complex functions
$mostComplex = $components | Where-Object { $_.complexity -eq $maxComplexity } | Select-Object -First 3
Write-Host "`nMost complex functions:" -ForegroundColor Red
foreach ($func in $mostComplex) {
    $filePath = ($func.location.file_path -split "\\")[-1]
    Write-Host "  - $($func.name) (complexity $($func.complexity)) in $filePath" -ForegroundColor White
}

# File distribution analysis
Write-Host "`n=== FILE DISTRIBUTION ===" -ForegroundColor Yellow
$fileGroups = $components | Group-Object { 
    $path = $_.location.file_path
    if ($path -match "\\src\\(.+)\.rs") { 
        $matches[1] -replace "\\", "/" 
    } else { 
        "other" 
    }
} | Sort-Object Count -Descending | Select-Object -First 20

foreach ($group in $fileGroups) {
    Write-Host "$($group.Name): $($group.Count) components" -ForegroundColor White
}

# Module categorization by RustChain features
Write-Host "`n=== FEATURE CATEGORIZATION ===" -ForegroundColor Yellow

$featureModules = @{
    "Core" = @("core/mod", "core/error", "core/memory", "core/chain", "core/agent")
    "LLM" = @("llm/mod", "llm/shimmy_provider", "llm/aws_bedrock_provider", "llm/azure_openai_provider", "llm/google_gemini_provider")
    "Tools" = @("tools/mod", "core/tools", "core/github_toolkit", "core/web_search_tools", "core/python_interpreter")
    "Security" = @("security/mod", "security/auth", "security/audit", "security/threat_detection", "security/encryption", "security/compliance")
    "SMT/Compliance" = @("smt/compliance_engine", "smt/contextlite_bridge", "smt/customer_tools", "smt/oscal_converter", "smt/solver", "smt/standards_compliance")
    "Engine" = @("engine/mod", "engine/chain_executor")
    "CLI" = @("cli/commands", "cli/handlers_pretty", "cli/interactive", "bin/rustchain-pretty")
    "Server" = @("server/mod")
    "ART" = @("art/performance", "art/ruler", "art/session", "art/tracker", "art/training")
    "Sandbox" = @("sandbox/mod")
    "Policy" = @("policy/mod")
    "Safety" = @("safety/mod")
    "RAG" = @("rag/mod", "core/document_loaders", "core/chroma_vector_store", "core/pinecone_vector_store")
}

foreach ($featureName in $featureModules.Keys) {
    $modulePatterns = $featureModules[$featureName]
    $featureComponents = $components | Where-Object {
        $filePath = $_.location.file_path
        $matched = $false
        foreach ($pattern in $modulePatterns) {
            if ($filePath -like "*$($pattern.Replace('/', '\'))*") {
                $matched = $true
                break
            }
        }
        $matched
    }
    
    $functionCount = ($featureComponents | Where-Object { $_.tags -contains "function" }).Count
    $structCount = ($featureComponents | Where-Object { $_.tags -contains "struct" }).Count
    $enumCount = ($featureComponents | Where-Object { $_.tags -contains "enum" }).Count
    $traitCount = ($featureComponents | Where-Object { $_.tags -contains "trait" }).Count
    $publicCount = ($featureComponents | Where-Object { $_.tags -contains "public" }).Count
    
    Write-Host "`n$featureName Feature:" -ForegroundColor Cyan
    Write-Host "  Total components: $($featureComponents.Count)" -ForegroundColor White
    Write-Host "  Functions: $functionCount | Structs: $structCount | Enums: $enumCount | Traits: $traitCount" -ForegroundColor White
    Write-Host "  Public API: $publicCount ($([math]::Round($publicCount/$featureComponents.Count*100, 2))%)" -ForegroundColor Green
}

# Test analysis
Write-Host "`n=== TEST COVERAGE ANALYSIS ===" -ForegroundColor Yellow
$testFiles = $components | Where-Object { 
    $_.location.file_path -like "*test*" -or 
    $_.tags -contains "test" -or
    $_.name -like "*test*"
}
$testCount = $testFiles.Count
$functionCount = ($components | Where-Object { $_.tags -contains "function" }).Count

Write-Host "Test-related components: $testCount" -ForegroundColor White
Write-Host "Total functions: $functionCount" -ForegroundColor White
Write-Host "Estimated test ratio: $([math]::Round($testCount/$functionCount*100, 2))%" -ForegroundColor Green

Write-Host "`n=== SUMMARY STATISTICS ===" -ForegroundColor Yellow
Write-Host "Total Components: $($components.Count)" -ForegroundColor Green
Write-Host "Functions: $(($components | Where-Object { $_.tags -contains 'function' }).Count)" -ForegroundColor White
Write-Host "Structs: $(($components | Where-Object { $_.tags -contains 'struct' }).Count)" -ForegroundColor White
Write-Host "Enums: $(($components | Where-Object { $_.tags -contains 'enum' }).Count)" -ForegroundColor White
Write-Host "Traits: $(($components | Where-Object { $_.tags -contains 'trait' }).Count)" -ForegroundColor White
Write-Host "Modules: $(($components | Where-Object { $_.tags -contains 'module' }).Count)" -ForegroundColor White

$totalLinesOfCode = ($components | Where-Object { $_.location.end_line -ne $null } | ForEach-Object { 
    $_.location.end_line - $_.location.start_line + 1 
} | Measure-Object -Sum).Sum

Write-Host "Estimated Lines of Code: $totalLinesOfCode" -ForegroundColor Cyan

Write-Host "`nAnalysis complete!" -ForegroundColor Green