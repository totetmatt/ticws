rem Usage run_server.bat <room> <handle>
start tic80.exe --skip --codeimport=showdown_%1_%2.dat --delay=5
ticws-server.exe %1 %2 showdown_%1_%2.dat
