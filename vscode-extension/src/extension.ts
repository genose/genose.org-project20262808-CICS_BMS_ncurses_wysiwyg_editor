import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { execSync } from 'child_process';

// Path to the Rust core binary (to be built)
const CORE_BIN = path.join(__dirname, '..', '..', 'target', 'release', 'cobol-bms-core');

export function activate(context: vscode.ExtensionContext) {
    console.log('COBOL BMS Tools extension activated');

    // Register preview command
    let previewCommand = vscode.commands.registerCommand('cobol-bms-tools.previewBMS', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor!');
            return;
        }

        const document = editor.document;
        if (document.languageId !== 'bms') {
            vscode.window.showErrorMessage('Current file is not a BMS file!');
            return;
        }

        const panel = vscode.window.createWebviewPanel(
            'bmsPreview',
            'BMS Map Preview',
            vscode.ViewColumn.Beside,
            {
                enableScripts: true,
                localResourceRoots: [vscode.Uri.file(context.extensionPath)],
            }
        );

        // Get BMS content
        const bmsContent = document.getText();
        
        // Try to use Rust core if available, otherwise use built-in parser
        let htmlContent: string;
        try {
            // Call Rust core for parsing and rendering
            const corePath = path.join(context.extensionPath, 'bin', 'cobol-bms-core');
            const tempFile = path.join(context.globalStorageUri.fsPath, 'temp.bms');
            fs.writeFileSync(tempFile, bmsContent);
            
            // For now, use the built-in renderer
            htmlContent = renderBmsPreview(bmsContent);
        } catch (e) {
            console.error('Failed to use Rust core, falling back to built-in:', e);
            htmlContent = renderBmsPreview(bmsContent);
        }

        panel.webview.html = getWebviewContent(htmlContent, context);
    });

    // Register generate COBOL command
    let generateCommand = vscode.commands.registerCommand('cobol-bms-tools.generateCobol', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor!');
            return;
        }

        const document = editor.document;
        if (document.languageId !== 'bms') {
            vscode.window.showErrorMessage('Current file is not a BMS file!');
            return;
        }

        // Save current file if not saved
        if (document.isDirty) {
            await document.save();
        }

        const filePath = document.fileName;
        const outputPath = filePath.replace(/\.bms$/i, '.cbl');

        try {
            // Call Rust CLI to generate COBOL
            const corePath = path.join(context.extensionPath, 'bin', 'cobol-bms');
            const command = `"${corePath}" generate "${filePath}" --output "${outputPath}"`;
            console.log('Running command:', command);
            
            execSync(command, { stdio: 'inherit' });
            
            // Open the generated file
            const doc = await vscode.workspace.openTextDocument(outputPath);
            await vscode.window.showTextDocument(doc);
            
            vscode.window.showInformationMessage(`Generated COBOL: ${path.basename(outputPath)}`);
        } catch (e) {
            console.error('Failed to generate COBOL:', e);
            vscode.window.showErrorMessage(`Failed to generate COBOL: ${e}`);
        }
    });

    context.subscriptions.push(previewCommand, generateCommand);
}

function getWebviewContent(htmlContent: string, context: vscode.ExtensionContext): string {
    const styleUri = vscode.Uri.file(path.join(context.extensionPath, 'webviews', 'styles.css'));
    const scriptUri = vscode.Uri.file(path.join(context.extensionPath, 'webviews', 'preview.js'));
    
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BMS Preview</title>
    <link rel="stylesheet" href="${styleUri}">
</head>
<body>
    ${htmlContent}
    <script src="${scriptUri}"></script>
</body>
</html>`;
}

function renderBmsPreview(bmsContent: string): string {
    // Simple built-in renderer (replaced by Rust core when available)
    const lines = bmsContent.split('\n');
    let mapName = 'Unknown';
    let mapset = 'Default';
    let size: [number, number] = [24, 80];
    const fields: Array<{
        name: string;
        pos: [number, number];
        length: number;
        attrb: string[];
        color: string | null;
    }> = [];

    // Parse BMS content (simplified)
    for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith('DFHMSD')) {
            const typeMatch = trimmed.match(/TYPE=(\w+)/i);
            const mapsetMatch = trimmed.match(/MAPSET=(\w+)/i);
            if (typeMatch) mapName = typeMatch[1].toUpperCase();
            if (mapsetMatch) mapset = mapsetMatch[1].toUpperCase();
        } else if (trimmed.startsWith('DFHMDI')) {
            const sizeMatch = trimmed.match(/SIZE=\((\d+),(\d+)\)/i);
            if (sizeMatch) {
                size = [parseInt(sizeMatch[1]), parseInt(sizeMatch[2])];
            }
        } else if (trimmed.startsWith('DFHMND') || trimmed.startsWith('DFHMDF')) {
            const posMatch = trimmed.match(/POS=\((\d+),(\d+)\)/i);
            const lengthMatch = trimmed.match(/LENGTH=(\d+)/i);
            const attrbMatch = trimmed.match(/ATTRB=\(([^)]+)\)/i);
            const colorMatch = trimmed.match(/COLOR=(\w+)/i);

            if (posMatch) {
                fields.push({
                    name: '',
                    pos: [parseInt(posMatch[1]), parseInt(posMatch[2])],
                    length: lengthMatch ? parseInt(lengthMatch[1]) : 1,
                    attrb: attrbMatch ? attrbMatch[1].split(',').map(a => a.trim().toUpperCase()) : [],
                    color: colorMatch ? colorMatch[1].toUpperCase() : null
                });
            }
        }
    }

    // Generate HTML grid
    let html = `<div class="bms-preview">
    <div class="header">
        <h1>BMS Map: ${mapName}</h1>
        <p>Mapset: ${mapset} | Size: ${size[0]}x${size[1]} | Fields: ${fields.length}</p>
    </div>
    <div class="grid" style="--rows: ${size[0]}; --cols: ${size[1]}">`;

    // Create grid cells
    for (let row = 1; row <= size[0]; row++) {
        for (let col = 1; col <= size[1]; col++) {
            let fieldClass = 'empty';
            let tooltip = '';

            for (const field of fields) {
                const [fieldRow, fieldCol] = field.pos;
                if (row === fieldRow && col >= fieldCol && col < fieldCol + field.length) {
                    if (field.attrb.includes('PROT')) {
                        fieldClass = 'field-prot';
                    } else if (field.attrb.includes('NUM')) {
                        fieldClass = 'field-num';
                    } else {
                        fieldClass = 'field';
                    }
                    tooltip = `Pos: (${fieldRow},${fieldCol}) Len: ${field.length} Attr: ${field.attrb.join(',')} Color: ${field.color || 'none'}`;
                    break;
                }
            }

            html += `<div class="cell ${fieldClass}" title="${tooltip}"></div>`;
        }
    }

    html += `</div>
    <div class="legend">
        <div class="legend-item"><span class="legend-color field"></span> Field (Input)</div>
        <div class="legend-item"><span class="legend-color field-prot"></span> Protected Field</div>
        <div class="legend-item"><span class="legend-color field-num"></span> Numeric Field</div>
        <div class="legend-item"><span class="legend-color empty"></span> Empty</div>
    </div>
</div>`;

    return html;
}

export function deactivate() {
    console.log('COBOL BMS Tools extension deactivated');
}
