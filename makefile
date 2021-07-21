libpath=/usr/lib#change this variable to your system's library path.
includepath=/usr/include#change this variable to your system's include path.
binpath=/usr/bin#change this variable to your system's binary path.
dlname=so#dynamic library file extension. Mac is dylib, Linux is so.
bins=lib/libEZLib.${dlname} installer/EZInstaller
libs=lib/libEZLib.${dlname}
CFLAGS=-std=c++17

# EZInstaller section
installer=installer/EZInstaller

usage:
	@echo 'Usage: "make [application_name]..."'
	@echo 'valid [application_name]:'
	@echo '	installinst: install EZInstaller'
	@echo '	installlib: install EZLib'
	@echo '	example: make installlib"'

lib/EZLib.o: lib/EZLib.cpp lib/preprocessor.hpp lib/EZLib
	g++ ${CFLAGS} -fPIC -c -o lib/EZLib.o lib/EZLib.cpp

${libs}: lib/EZLib.o
	g++ ${CFLAGS} -shared -o lib/libEZLib.${dlname} lib/EZLib.o

${installer}: installer/EZInstaller.cpp installer/preprocessor.hpp
	g++ ${CFLAGS} -o ${installer} installer/EZInstaller.cpp -lEZLib -lcurl

installlib: ${libs}
	mkdir -p ${includepath}/EZModpacker
	mv lib/libEZLib.${dlname} ${libpath}/
	cp lib/EZLib ${includepath}/EZModpacker/

installinst: ${installer}
	mv installer/EZInstaller ${binpath}/

uninstalllib:
	rm ${libpath}/libEZLib.${dlname} ${includepath}/EZModpacker/EZLib

uninstallinst:
	rm ${binpath}/EZInstaller

uninstall:
	rm ${libpath}/libEZLib.${dlname} ${includepath}/EZModpacker/EZLib ${binpath}/EZInstaller

clean:
	rm lib/EZLib.o
