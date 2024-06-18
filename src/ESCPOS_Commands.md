# ESC/POS Commands for the TM-T88III Printer

This lists the ESC∕POS commands for the TM-T88III printer along with some details about them

## Command List

| Command | Classification | Name | Function type |
| --- | --- | --- | --- |
| HT | Exectuing command | Horizontal Tab | Print position commands |
| LF | Exectuing command | Print and line feed | Print commands | 
| FF (in page mode) | Exectuting command | Print and return to standard mode | Print commands |
| CR | Exectuting command | Print and carriage return | Print commands |
| CAN | Exectuting command | Cancel print data in page mode | Character commands |
| DLE EOT | Exectuting command | Real-time status transmission | Status commands |
| DLE ENQ | Exectuting command | Real-time request to printer | Miscellaneous commands |
| DLE DC4 (fn = 1) | Executing + setting | Generate pulse at real-time | Miscellaneous commands |
| DLE DC4 (fn = 8) | Executing + setting | Clear buffer | Miscellaneous commands |
| ESC FF | Executing command | Print data in page mode | Print commands |
| ESC SP | Setting command | Set right-side character spacing | Character commands |
| ESC ! | Setting command | Select print mode(s) | Character commands |
| ESC $ | Executing command  | Set absolute print position | Print position commands |
| ESC % | Setting Commaand | Select/cancel user-defined characters | Character commands |
| ESC & | Setting command | Define user-defined characters | Character commands |
| ESC * | Executing command | Select bit-image mode | Bit-image commands |
| ESC - | Setting command | Turn underline mode on/off | Character commands |
| ESC 2 | Executing command | Select default line spacing | Line spacing commands |
| ESC 3 | Setting command | Set line spacing | Line spacing commands |
| ESC = | Setting command | Select peripheral device | Miscellaneous commands |
| ESC ? | Setting command | Cancel user-defined characters | Character commands |
| ESC @ | Executing + setting | Initialize printer | Miscellaneous commands |
| ESC D | Setting command | Set horizontal tab positions | Print position commands |
| ESC E | Setting command | Turn emphasized mode on/off | Character commands |
| ESC G | Setting command | Turn double-strike mode on/off | Character commands |
| ESC J | Executing command | Print and feed paper | Print commands |
| ESC L | Executing command | Select page mode | Miscellaneous commands |
| ESC M | Setting command | Select character font | Character commands |
| ESC R | Setting command | Select an international character set | Character commands |
| ESC S | Setting command | Set standard mode | Miscellaneous commands |
| ESC T | Setting command | Select print direction in page mode | Print position commands |
| ESC V | Setting command | Turn 90° clockwise rotation mode on/off | Character commands |
| ESC W | Setting command | Set printing area in page mode | Print position commands |
| ESC \ | Executing command | Set relative print poisition | Print position commands |
| ESC a | Setting command | Select justification | Print position commands |
| ESC c 3 | Setting command | Select paper sensor to output paper-end signals | Paper sensor commands |
| ESC c 4 | Setting command | Select paper sensor to stop printing | Paper sensor commands |
| ESC c 5 | Setting command | Enable/disable panel buttons | Panel button commands |
| ESC d | Executing command | Print and feed *n* lines | Print commands |
| ESC p | Executing command | Generate pulse | Miscellaneous commands |
| ESC t | Setting command | Select character code table | Character commands |
| ESC { | Setting command | Turn upside-down printing mode on/off | Character commands |
| GS ! | Setting command | Select character size | Character commands |
| GS $ | Executing command | Set absolute vertical print position in page mode | Print position commands |
| GS ( a | Executing command | Execute test print | Miscellaneous commands |
| GS : | Executing + setting | Start∕end macro definition | Macro function commands |
| GS B | Setting command | Turn white/black reverse printing mode on/off | Character commands |
| GS H | Setting command | Select printing position of HRI characters | Bar code commands |
| GS I | Executing command | Transmit printer ID | Miscellaneous commands |
| GS L | Setting command | Set left margin | Print position commands |
| GS P | Setting command | Set horizontal and vertical motion units | Miscellaneous commands |
| GS V | Executing command | Select cut mode and cut paper | Mechanism control commands |
| GS W | Setting command | Set print area width | Print position commands |
| GS \ | Executing command | Set relative vertical print position in page mode | Print position commands |
| GS ^ | Exectuting command | Execute macro | Macro function commands |
| GS a | Executing + setting | Enable/disable automatic status back | Status commands |
| GS b | Setting command | Turn smooting mode on / off | Character commands |
| GS f | Setting command | Select font for HRI characters | Bar code commands |
| GS h | Setting command | Set bar code height | Bar code commands |
| GS k | Executing command | Print bar code | Bar code commands |
| GS r | Executing command | Transmit status | Status commands |
| GS w | Setting command | Set bar code width | Bar code commands |
| FS ! | Setting command | Set print mode(s) for Kanji characters | Kanji commands |
| FS & | Setting command | Select Kanji character mode | Kanji commands |
| FS ( A | Setting command | Define character effects of Kanji characters | Kanji commands |
| FS - | Setting command | Turn underline mode on/off for Kanji characters | Kanji commands |
| FS . | Setting command | Cancel Kanji character mode | Kanji commands |
| FS 2 | Setting command | Define user-defined Kanji characters | Kanji commands |
| FS C | Setting command | Select Kanji character code system | Kanji commands |
| FS W | Setting command | Turn quadruple-size mode on/off for Kanji characters | Kanji commands |
| FS ? | Setting command | Cancel user-defined Kanji characters | Kanji commands |

## Print Commands 

Here follows a more detailed description of the print commands

### LF - Print and line feed
#### Format
| ASCII | Hexadecimal | Decimal |
| --- | --- | --- |
| LF | 0A | 10 |
#### Range
None
#### Default
None
#### Description
Prints the data in the print buffer and feeds one line based on the current line spacing.
#### Notes
⋅The amount of paper fed per line is based on the value set using the line spacing command(**ESC 2** or **ESC 3**).
⋅After printing, the print position moves to the beginning of the line. When a left margin is set in standard mode, the position of the left margin is the beginning of the line.
⋅When this command is processed in page mode, only the print position moves, and the printer does not perform actual printing.

### FF (in page mode) - Print and return to standard mode
#### Format
| ASCII | Hexadecimal | Decimal |
| --- | --- | --- |
| FF | 0C | 12 |
#### Range
None
#### Default
None
#### Description
In page mode, print al lthe data in the print buffer collectively and switches from page mode to standard mode.
#### Notes
⋅This command is enabled only in page mode. See **FF** (in standard mode) to use this command in standard mode.
⋅The data is deleted in the print area after being printer.
⋅ This command returns the values set up **ESC W** to the defatul values
⋅ The value set but **ESC T** is maintained.
⋅ After printing, the print position moves to the beginning of the line. When a left margin is set, the position of the left margin is the beginning of the line.

### CR - Print and carriage return
#### Format
| ASCII | Hexadecimal | Decimal |
| --- | --- | --- |
| CR | 0D | 13 |
#### Range
None
#### Default
None
#### Description
When the auto line feed is enabled, exectues printing and one line feed as **LF**. When auto line feed is disabled, this command is ignored
#### Notes
⋅ After printing, the print position moves to the beginning of the line. When a left margin is set in standard mode, the position of the left margin is the beginning of the line.
⋅ When this command is processed in page mode, only the print position moves, and the printer does not perform actual printing.

### ESC FF - Print data in page mode
#### Format
| ASCII | Hexadecimal | Decimal |
| --- | --- | --- |
| ESC FF | 1B 0C | 27 12 |
#### Range
None
#### Default
None
#### Description
In page mode, prints the data in the print buffer collectively.
#### Notes
⋅ This command is enabled only in page mode. Page mode can be selected by **ESC L**.
⋅ After printing, the printer does not clear the buffered data, the print position, or values set by other commands.
⋅ The printer returns to standard mode with **FF**, **ESC C**, and **ESC @**: When it returns to standard mode by **ESC @**, all settings are canceled.
⋅ This command is used when the data in page mode is printed repeatedly.

### ESC J - Print and feed paper
#### Format
| ASCII | Hexadecimal | Decimal |
| --- | --- | --- |
| ESC J | 1B 4A | 27 74 |
#### Range
0 ≤ n ≤ 255
#### Default
None
#### Description
Prints the data in the print buffer and feeds the paper **n** * (vertical or horizontal motion unit)
#### Notes
⋅ The maximum paper feed amout is 1016 mm {40 incesh}. If the specified amount exceeds the maximum, the printer feeds the paper to the maximum amount.
⋅ 