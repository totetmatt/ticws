rem Usage run_client.bat <room> <handle>
start tic80.exe --skip --codeexport=showdown_%1_%2.dat --delay=5
ticws-client.exe %1 %2 showdown_%1_%2.dat
