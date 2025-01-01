TICEXE=./tic80-macos-arm64/tic80
CLIENT_EXE=./target/release/ticws-client
SHOWDOWN_FILE="client_showdown_$1_$2.dat"

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

if [ ! -x "$CLIENT_EXE" ]; then
    echo "$CLIENT_EXE not found / not executable"
    exit 1
fi

touch $SHOWDOWN_FILE
$TICEXE --fft --skip "--codeexport=$SHOWDOWN_FILE" --delay=5 &
$CLIENT_EXE "$1" "$2" "$SHOWDOWN_FILE"
