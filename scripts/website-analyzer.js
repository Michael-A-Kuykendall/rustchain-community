const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');

async function analyzeWebsite() {
    const browser = await puppeteer.launch({ 
        headless: false,
        defaultViewport: { width: 1920, height: 1080 }
    });
    
    try {
        const page = await browser.newPage();
        
        // Navigate to the website
        console.log('📱 Navigating to website...');
        await page.goto('http://localhost:8081', { waitUntil: 'networkidle0' });
        
        // Wait for 3D animations to load
        await new Promise(resolve => setTimeout(resolve, 3000));
        
        // Take full page screenshot
        console.log('📸 Taking screenshot...');
        const screenshotPath = path.join(__dirname, '../mission-outputs/website-screenshot.png');
        await page.screenshot({ 
            path: screenshotPath, 
            fullPage: true,
            type: 'png'
        });
        
        // Analyze accessibility
        console.log('🔍 Running accessibility analysis...');
        
        // Inject axe-core for accessibility testing
        await page.addScriptTag({
            url: 'https://unpkg.com/axe-core@4.7.0/axe.min.js'
        });
        
        const accessibilityResults = await page.evaluate(async () => {
            const results = await axe.run();
            return {
                violations: results.violations.map(v => ({
                    id: v.id,
                    impact: v.impact,
                    description: v.description,
                    help: v.help,
                    nodes: v.nodes.length
                })),
                passes: results.passes.length,
                incomplete: results.incomplete.length
            };
        });
        
        // Analyze color contrast
        console.log('🎨 Analyzing color contrast...');
        const colorAnalysis = await page.evaluate(() => {
            const elements = document.querySelectorAll('*');
            const colorIssues = [];
            
            elements.forEach((el, index) => {
                const styles = window.getComputedStyle(el);
                const color = styles.color;
                const backgroundColor = styles.backgroundColor;
                const text = el.textContent?.trim();
                
                if (text && text.length > 0 && text.length < 200) {
                    colorIssues.push({
                        element: el.tagName.toLowerCase(),
                        text: text.substring(0, 50),
                        color: color,
                        backgroundColor: backgroundColor,
                        className: el.className
                    });
                }
            });
            
            return colorIssues.slice(0, 50); // Limit results
        });
        
        // Extract button information
        console.log('🔘 Analyzing buttons...');
        const buttonAnalysis = await page.evaluate(() => {
            const buttons = document.querySelectorAll('button, .btn, [role="button"]');
            return Array.from(buttons).map(btn => ({
                text: btn.textContent?.trim(),
                className: btn.className,
                styles: {
                    color: window.getComputedStyle(btn).color,
                    backgroundColor: window.getComputedStyle(btn).backgroundColor,
                    border: window.getComputedStyle(btn).border
                }
            }));
        });
        
        // Save analysis results
        const analysisResults = {
            timestamp: new Date().toISOString(),
            url: 'http://localhost:8081',
            accessibility: accessibilityResults,
            colorAnalysis: colorAnalysis,
            buttonAnalysis: buttonAnalysis,
            recommendations: [
                'Add text stroke for better readability',
                'Improve color contrast ratios',
                'Test with colorblind simulation',
                'Redesign top navigation buttons'
            ]
        };
        
        const resultsPath = path.join(__dirname, '../mission-outputs/website-analysis.json');
        fs.writeFileSync(resultsPath, JSON.stringify(analysisResults, null, 2));
        
        console.log('✅ Analysis complete!');
        console.log(`📸 Screenshot saved to: ${screenshotPath}`);
        console.log(`📊 Analysis saved to: ${resultsPath}`);
        
        // Generate colorblind simulations
        console.log('🌈 Generating colorblind simulations...');
        
        // Protanopia (red-blind)
        await page.emulateVisionDeficiency('protanopia');
        await page.screenshot({ 
            path: path.join(__dirname, '../mission-outputs/website-protanopia.png'), 
            fullPage: true 
        });
        
        // Deuteranopia (green-blind)
        await page.emulateVisionDeficiency('deuteranopia');
        await page.screenshot({ 
            path: path.join(__dirname, '../mission-outputs/website-deuteranopia.png'), 
            fullPage: true 
        });
        
        // Tritanopia (blue-blind)
        await page.emulateVisionDeficiency('tritanopia');
        await page.screenshot({ 
            path: path.join(__dirname, '../mission-outputs/website-tritanopia.png'), 
            fullPage: true 
        });
        
        // Reset vision
        await page.emulateVisionDeficiency('none');
        
        console.log('🎨 Colorblind simulations complete!');
        
        return analysisResults;
        
    } finally {
        await browser.close();
    }
}

// Install puppeteer if not available
async function installDependencies() {
    const { execSync } = require('child_process');
    try {
        require('puppeteer');
        console.log('📦 Puppeteer already installed');
    } catch (e) {
        console.log('📦 Installing Puppeteer...');
        execSync('npm install puppeteer', { stdio: 'inherit' });
    }
}

async function main() {
    try {
        await installDependencies();
        const results = await analyzeWebsite();
        
        console.log('\n🔍 ACCESSIBILITY VIOLATIONS FOUND:');
        results.accessibility.violations.forEach(v => {
            console.log(`❌ ${v.id}: ${v.description} (${v.impact} impact, ${v.nodes} elements)`);
        });
        
        console.log(`\n✅ ${results.accessibility.passes} accessibility checks passed`);
        console.log(`⚠️ ${results.accessibility.incomplete} checks incomplete`);
        
        console.log('\n🔘 BUTTON ANALYSIS:');
        results.buttonAnalysis.forEach((btn, i) => {
            if (btn.text) {
                console.log(`Button ${i + 1}: "${btn.text}" - ${btn.styles.color} on ${btn.styles.backgroundColor}`);
            }
        });
        
    } catch (error) {
        console.error('❌ Error analyzing website:', error);
    }
}

if (require.main === module) {
    main();
}

module.exports = { analyzeWebsite };