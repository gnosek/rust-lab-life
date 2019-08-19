cmake_minimum_required(VERSION 3.12)
project(life-cpp)

option(FFI_DIR "Path to Rust board implementation via FFI (empty to disable)" "")

set(BOARD_SOURCES
		board.cpp
		board.h
)

if(NOT FFI_DIR)
	list(APPEND BOARD_SOURCES ffi-export.cpp)
endif()

add_library(board ${BOARD_SOURCES})

set_property(TARGET board PROPERTY CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS -fPIC)

if (FFI_DIR)
	set(BOARD_HEADER ffi-import.h)
	set(BOARD_LIB life_rs pthread dl)
	add_definitions(-DUSE_FFI)
	link_directories(${FFI_DIR})
else ()
	set(BOARD_HEADER board.h)
	set(BOARD_LIB board)
endif ()

add_executable(
		life-cpp
		${BOARD_HEADER}
		display.h
		main.cpp
)
set_property(TARGET life-cpp PROPERTY CXX_STANDARD 17)

if (USE_FFI)
	set_target_properties(life-cpp PROPERTIES LINK_SEARCH_START_STATIC 1)
	set_target_properties(life-cpp PROPERTIES LINK_SEARCH_END_STATIC 1)
endif ()

target_link_libraries(life-cpp ${BOARD_LIB})
