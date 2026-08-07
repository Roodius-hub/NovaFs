import { invoke } from "@tauri-apps/api/core";
import type { FileNode } from "../src/types/filesystem";
import { useState } from "react";

export default function FileExplorer() {
    const [tree, setTree] = useState<FileNode[]>([]);

    async function scan() {
        try {
            const result = await invoke<FileNode[]>('scan_dir', {
                path: ".",
            });
            setTree(result)
        } catch (err) {
            console.error(err);
        }
    }
    
    return (
        <div className="flex-1 p-4">
            <button onClick={scan}>scan</button>
            <pre>{JSON.stringify(tree, null, 2)}</pre>
        </div>
    );
}