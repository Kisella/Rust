@echo off
powershell -Command "$currentDir = Get-Location; Get-ChildItem -Path $currentDir -Directory -Recurse -Filter '.git' -Force | Where-Object { $_.FullName -ne \"$currentDir\\.git\" } | ForEach-Object { Remove-Item -Path $_.FullName -Recurse -Force }"
echo .git of all subdirectories has been cleaned up! 
pause