cmake_minimum_required(VERSION 3.25.1)
project(bakasable-rs VERSION 1.0.0)

include(FetchContent)

FetchContent_Declare(corrosion
        GIT_REPOSITORY https://github.com/Bktero/corrosion.git
        GIT_TAG fix-all-cbindgen
)

FetchContent_MakeAvailable(corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

corrosion_experimental_cbindgen(TARGET bakasable_rs HEADER_NAME "bakasable.h")

corrosion_install(TARGETS bakasable_rs)