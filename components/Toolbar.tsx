export default function Toolbar() {
    return (
        <div className="h-14 border-b border-zinc-800 flex items-center gap-3 px-4">
            <button>Scan</button>
            <button>Refresh</button>
            <button>Create File</button>
            <button>Create Folder</button>
            <button>Delete</button>

            <input
                type="text"
                placeholder="Search..."
                className="ml-auto px-2 py-1 rounded bg-zinc-800 outline-none"
            />
        </div>
    );
}