# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./vhs/mot.tape`
Output "target/mot.gif"
Set Theme "Catppuccin Frappe"
Set Shell zsh
Set Width 1200
Set Height 768
Set FontSize 16
Set BorderRadius 10
Set TypingSpeed 300ms
Set Padding 20
Type "mot"
Enter
Sleep 2s
Set TypingSpeed 500ms
Down 5
Type " "
Type " "
Type " "
Up 1
Sleep 1s
Down 3
Sleep 1s
Type " "
Up 1
Sleep 1s
Type "/"
Sleep 1s
Type "tags"
Sleep 1s
Type " "
Sleep 1s
Escape
Sleep 1s
Type "w"
Set TypingSpeed 100ms
Type "cat mot.out.yaml | more"
Sleep 1s
Set TypingSpeed 1s
Enter
Sleep 2s
Type " "
Sleep 2s
Escape