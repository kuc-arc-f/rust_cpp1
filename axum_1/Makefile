CXX = clang++
CXXFLAG_1= -I./include -I/prog/vcpkg/installed/x64-windows/include
CXXFLAG_LIB_1= -L/prog/vcpkg/installed/x64-windows/lib

CXXFLAGS = -shared -fPIC -std=c++17 -I./include -lsqlite3
#CXXFLAGS = -shared -fPIC -std=c++17 $(CXXFLAG_1) $(CXXFLAG_LIB_1)

TARGET = libsample.so
all: $(TARGET)

$(TARGET): sample.o
	$(CXX) $(CXXFLAGS) sample.o -o $(TARGET)

sample.o: sample.cpp
	$(CXX) $(CXXFLAGS) -c sample.cpp

clean:
	rm *.o $(TARGET)
