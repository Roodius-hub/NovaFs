import Toolbar from "../components/Toolbar";
import Sidebar from "../components/Sidebar";
import FileExplorer from "../components/FileExplorer";
import Properties from "../components/Properties";
import StatusBar from "../components/StatusBar";

export default function Home() {
    return (
        <>
            <Toolbar />

            <div className="flex flex-1">
                <Sidebar />

                <FileExplorer />

                <Properties />
            </div>

            <StatusBar />
        </>
    );
}