Output assets/rtotp-demo.gif

Set Shell "bash"
Set FontSize 16
Set Width 1120
Set Height 720
Set Padding 24
Set TypingSpeed 35ms
Set Theme "Catppuccin Mocha"

Hide
Type "export HOME=$(mktemp -d)" Enter
Type "echo W3siaXNzdWVyIjoia2VlcCIsImFjY291bnRfbmFtZSI6Im93bmVyIiwic2VjcmV0IjoiSkJTV1kzRFBFSFBLM1BYUEpCU1dZM0RQRUhQSzNQWFAifV0= | base64 -d > $HOME/.rtotplist" Enter
Show
Ctrl+L

Type "rtotp list" Enter
Sleep 900ms

Type "rtotp 1" Enter
Sleep 900ms

Type "rtotp qr 1" Enter
Sleep 4s
