import { FolderList } from './FolderList';
import './Sidebar.css';

export function Sidebar() {
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1>SceneBrowser</h1>
        <p className="version">v0.1.0</p>
      </div>
      <FolderList />
    </aside>
  );
}
