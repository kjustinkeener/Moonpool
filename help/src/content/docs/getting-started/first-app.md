---
title: "Add and run your first app in Moonpool"
description: "Go from first launch to one running app of your own in a few minutes: add it, start it, stop it, and find the hub and this help again later."
---

## 1. Start Moonpool

On Windows, run `moonpool.exe` and click **Install Moonpool** (see
[Installing](/getting-started/install/)). On Linux, start the AppImage or the installed
package.

The first time Moonpool runs it fills the sidebar with example apps (Notepad on Windows, a
shell, a small web server and the bundled dashboards). They run as they are (the web server
needs Python), so you can try them, then edit or delete them. It also puts an icon in the system tray. On Windows, if
you do not see the icon, click the **^** arrow at the right of the taskbar.

## 2. Add your app

1. Open the **...** menu at the top of the sidebar and choose **Add app**.
2. Enter a **name**. The group starts as `Web apps`; keep it or pick another.
3. Keep **type** as `web` for a dev server.
4. Set **cwd** to your project folder and **command** to what you type to start it, for
   example `npm run dev`.
5. Set **port** to the port it listens on, and **url** to the page to open.
6. Save.

The details of every field are in [Adding apps](/apps/add-an-app/).

## 3. Start it

Click the app's **Launch** button (the play icon on its row). Its terminal tab opens and
shows the output. The status dot pulses while the app is starting, then turns solid once its
port answers. If **openBrowser** is on, the page opens.

Clicking the app's name only opens its terminal tab. It never starts the app.

## 4. Stop it

Click the **Stop** button (the square) on the row. The dot turns grey.

If something stays running after Stop, see [Stop and restart](/apps/stop-and-restart/).

## Let an agent do it

With no tab open, the CLI pane shows a **Copy prompt** button. Paste the prompt into an AI
agent and it finds your apps and adds them. See
[AI agents: quick start](/automation/quick-start/).

## Finding the hub later

- Left-click the tray icon to show the hub. Right-click it for a menu with **Show Moonpool**
  and **Quit**.
- By default, closing the window quits Moonpool. Turn on **Close to tray** in Settings to
  hide it to the tray instead and keep it running. See
  [Tray, close and minimize](/using/tray-and-closing/).

## Getting help

**Help**, in the **...** menu at the top of the sidebar, opens this help in its own window.
It works offline and always matches the version you run.

![Help window with the section navigation outlined on the left and a page on the right](../../../assets/screenshots/help-window.png)

## Next

- [Adding apps](/apps/add-an-app/)
- [Troubleshooting](/support/troubleshooting/)
