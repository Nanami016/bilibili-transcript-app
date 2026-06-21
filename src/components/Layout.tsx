import { Outlet } from "react-router-dom";
import Sidebar from "./Sidebar";
import TaskToast from "./TaskToast";

function Layout() {
  return (
    <div className="app-layout">
      <Sidebar />
      <main className="main-content">
        <Outlet />
      </main>
      <TaskToast />
    </div>
  );
}

export default Layout;
