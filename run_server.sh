TICEXE=./tic80
SERVER_EXE=./ticws-server
SHOWDOWN_FILE="server_showdown_$1_$2.dat"

if [ -z "$1" ] || [ -z "$2" ] ; then
    echo "Usage: $0 <room> <handle>"
    exit 1
fi

if [ ! -x $TICEXE ]; then
    TIC_CHECK=`command -v tic80`
    if [ -z "$TIC_CHECK" ]; then
        echo "$TICEXE not found and tic80 not in path"
        exit 1
    else
        TICEXE="tic80"
    fi
fi


if [ ! -x "$SERVER_EXE" ]; then
    echo "$SERVER_EXE not found / not executable"
    exit 1
fi

$TICEXE --skip "--codeimport=$SHOWDOWN_FILE" --fft --delay=5 &
$SERVER_EXE "$1" "$2" "$SHOWDOWN_FILE"
