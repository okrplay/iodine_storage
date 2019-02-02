if [ "$TRAVIS_OS_NAME" == "windows" ];
then
    choco install openssl.light --params "/InstallDir:C:\Users\travis\openssl_lib"
    exec shell
fi