# VHS tape for rtotp demo GIF. Regenerate with: vhs demo.tape
Output assets/demo.gif

Set Shell "bash"
Set FontSize 26
Set Width 1120
Set Height 640
Set Padding 28
Set Theme "Dracula"
Set TypingSpeed 55ms

# Setup (hidden): isolated HOME so the real ~/.rtotplist is untouched, clean prompt.
Hide
Type "export HOME=/tmp/rtotp-vhs; rm -rf /tmp/rtotp-vhs; mkdir -p /tmp/rtotp-vhs; export PATH=/Users/eun2ce/.cargo/bin:$PATH; export PS1='$ '; clear"
Enter
Sleep 500ms
Show

Sleep 800ms

# Register a secret
Type "rtotp add"
Enter
Sleep 700ms
Type "keep"
Enter
Sleep 500ms
Type "owner"
Enter
Sleep 500ms
Type "JBSWY3DPEHPK3PXPMFRGGZDFMZTWQ2LK"
Enter
Sleep 1200ms

# List and generate a code
Type "rtotp list"
Enter
Sleep 1200ms
Type "rtotp 1"
Enter
Sleep 2500ms
