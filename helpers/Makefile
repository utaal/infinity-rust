
########################################

SOURCE_FILES		= 	src/infinityhelpers/helpers.cpp

HEADER_FILES		= 	src/infinityhelpers/helpers.h
				
########################################

PROJECT_NAME		= libinfinityhelpers

########################################

SOURCE_FOLDER		= src
BUILD_FOLDER		= build
RELEASE_FOLDER		= release
INCLUDE_FOLDER		= include

########################################
			
OBJECT_FILES		= $(patsubst $(SOURCE_FOLDER)/%.cpp,$(BUILD_FOLDER)/%.o,$(SOURCE_FILES))
SOURCE_DIRECTORIES	= $(dir $(HEADER_FILES))
BUILD_DIRECTORIES	= $(patsubst $(SOURCE_FOLDER)/%,$(BUILD_FOLDER)/%,$(SOURCE_DIRECTORIES))

########################################

COMPILER_FLAGS 		= -O3 -std=c++0x -fPIC

########################################

all: library

########################################

$(BUILD_FOLDER)/%.o: $(SOURCE_FILES) $(HEADER_FILES)
	mkdir -p $(BUILD_FOLDER)
	mkdir -p $(BUILD_DIRECTORIES)
	g++ $(COMPILER_FLAGS) -c $(SOURCE_FOLDER)/$*.cpp -I $(SOURCE_FOLDER) -I ../vendor/rdma-database-engine/code/net/infinity/release/include -o $(BUILD_FOLDER)/$*.o

########################################

library: $(OBJECT_FILES)
	mkdir -p $(RELEASE_FOLDER)
	ar rvs $(RELEASE_FOLDER)/$(PROJECT_NAME).a $(OBJECT_FILES)
	rm -rf $(RELEASE_FOLDER)/$(INCLUDE_FOLDER)
	cp --parents $(HEADER_FILES) $(RELEASE_FOLDER)
	mv $(RELEASE_FOLDER)/$(SOURCE_FOLDER)/ $(RELEASE_FOLDER)/$(INCLUDE_FOLDER)

########################################

clean:
	rm -rf $(BUILD_FOLDER)
	rm -rf $(RELEASE_FOLDER)

