update-midas:
  @cargo run > /dev/null

  @rm -rf ../bitcoin-rpc-midas/*
  @cp -r bitcoin-rpc-midas/* ../bitcoin-rpc-midas/

  @cd ../bitcoin-rpc-midas && \
    git add . && \
    git commit -m "Update: Regenerated `bitcoin-rpc-midas` from `pipeline`" > /dev/null 2>&1 || exit 0

  @cd ../bitcoin-rpc-midas && \
    git --no-pager show --oneline HEAD
