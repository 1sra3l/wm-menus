# Window Manager menus
A simple little program I made for my own use.
This creates menus for JWM and Openbox

Still a work in progress, it does not support the XDG standards for menus, only builds a full menu from any dekstop file in a normal location

CLI usage:

```
wm-menus [OPTIONS]
    -p || --places || places      Show a places menu for JWM
    -a || --apps || apps          Show an application menu for JWM
    -o || --openbox || openbox    Show an application menu for openbox

```

All icons in the app menu have paths, to ensure it works with openbox
The menu constructed for JWM is something like:
```xml
<JWM>
   <Menu label="AudioVideo" icon="/usr/share/icons/Numix-Circle/48/apps/multimedia-volume-control.svg" tooltip="AudioVideo" height="0">
      <Program icon="/usr/share/icons/Numix-Circle/48/apps/audacious.svg" label="Audacious" tooltip="Listen to music">audacious</Program>
      <!-- etc...-->
    </Menu>
    <Menu label="Graphics" icon="/usr/share/icons/Numix-Circle/48/apps/accessories-painting.svg" tooltip="Graphics" height="0">
      <Program icon="/usr/share/icons/Numix-Circle/48/apps/inkscape.svg" label="Inkscape" tooltip="Create and edit Scalable Vector Graphics images">inkscape</Program>
      <!-- etc...-->
    </Menu>
    <!-- etc...-->
    <Program icon="reload" label="Update Menus">jwm -reload</Program>
</JWM>
```

And openbox looks like:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<openbox_menu xmlns="http://openbox.org/3.4/menu">
<menu id="AudioVideo-id" label="AudioVideo" icon="/usr/share/icons/Numix-Circle/48/apps/multimedia-volume-control.svg">
	<item label="Audacious" icon="/usr/share/icons/Numix-Circle/48/apps/audacious.svg">
		<action name="Execute">
			<command>audacious</command>
			<startupnotify>
				<enabled>yes</enabled>
			</startupnotify>
		</action>
	</item>
	<!-- etc...-->
</menu>
<menu id="Graphics-id" label="Graphics" icon="/usr/share/icons/Numix-Circle/48/apps/accessories-painting.svg">
	<item label="Inkscape" icon="/usr/share/icons/Numix-Circle/48/apps/inkscape.svg">
		<action name="Execute">
			<command>inkscape</command>
			<startupnotify>
				<enabled>yes</enabled>
			</startupnotify>
		</action>
	</item>
	<!-- etc...-->
</menu>
<!-- etc...-->
<menu id="root-menu" label="Openbox">
	<menu id="AudioVideo-id" />
	<menu id="Development-id" />
	<menu id="Game-id" />
	<menu id="Graphics-id" />
	<menu id="Network-id" />
	<menu id="Office-id" />
	<menu id="Settings-id" />
	<menu id="System-id" />
	<menu id="Utility-id" />

	<separator />
	<item label="Log Out">
		<action name="Exit">
			<prompt>yes</prompt>
		</action>
	</item></menu>
</openbox_menu>

```

It also creates a "Places" menu for JWM something like:
```xml
<JWM>
    <Program icon="user-home" label="Home">xdg-open /home/user</Program>
    <Menu label="Music" icon="folder-music" height="0">
        <Program icon="folder-music" label="Music">xdg-open /home/user/Music</Program>
        <!-- other sub directories -->
    </Menu>
    <Menu label="Downloads" icon="folder-download" height="0">
        <Program icon="folder-download" label="Downloads">xdg-open /home/user/Downloads</Program>
    </Menu>
    <Menu label="Documents" icon="folder-documents" height="0">
        <Program icon="folder-documents" label="Documents">xdg-open /home/user/Documents</Program>
        <!-- other sub directories -->
    </Menu>
<!-- etc...-->
   <Program icon="user-trash" label="Trash">xdg-open /home/user/.local/share/Trash/files</Program>
   <Program icon="reload" label="Update Menus">jwm -reload</Program>
</JWM>

```
