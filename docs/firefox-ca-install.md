# Import CA into Firefox

First, open the menu at the top right of Firefox and click "Preferences". This will open up a new tab with all of Firefox's preferences available.

![Image showing where to click to open Firefox Preferences](./images/firefox-menu.png)

On the left side of the Preferences screen, click on "Privacy & Security".

![Image showing where to click to switch to Privacy & Security Preferences](./images/firefox-preferences-sidebar.png)

Scroll to the bottom and click the button that says "View Certificates...". This will open a modal.

![Image showing where to click to switch to open the certificates modal](./images/firefox-security-preferences.png)

In the modal, switch to the "Authorities" tab. Then click the "Import..." button.

![Image showing where to click to switch to open start importing a CA](./images/firefox-authorities-modal.png)

Select the file at `$DEVCA_HOME/ca/cert.pem` and import it. If you don't know this location, you can run `devca path-to ca --cert` to find it.

That's it! Firefox should now trust any certificates generated by `devca`. Note that if you run the `regen` command, you will have to follow these steps again with the new CA certificate.
