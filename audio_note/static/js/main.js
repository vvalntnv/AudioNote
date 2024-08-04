import { sidebarToggle } from "./components/sidebarToggle.js";
import { logoutUser } from "./components/logoutButton.js";
import { applyCloseToMessages } from "./components/messages.js";

globalThis.sidebarToggle = sidebarToggle;
globalThis.logoutUser = logoutUser;

applyCloseToMessages();
