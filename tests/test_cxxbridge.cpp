#define BOOST_TEST_MODULE test_cxxbridge
#include <boost/test/included/unit_test.hpp>

#include <boost/filesystem.hpp>

#include <elfshaker-cxxbridge/lib.h>
#include <rust/cxx.h>
#include <vector>
#include <iostream>
#include <pre/file/string.hpp>

namespace fs = boost::filesystem;

BOOST_AUTO_TEST_CASE(store_with_separate_worktree_smoke_test) {
  auto temp_test_path = fs::temp_directory_path() / "elfshkr-test" / fs::unique_path();
  std::string worktree_path = temp_test_path.string();
  worktree_path += "/worktree";

  std::string elfshaker_data_dir = temp_test_path.string();
  elfshaker_data_dir += "/elfshaker_data";

  try {
    elfshaker::init_elfshaker_store( elfshaker_data_dir, worktree_path);
  } catch(const rust::Error &e) {
    std::cout << e.what() << std::endl;
  }

  pre::file::from_string((fs::path{worktree_path} / "README.md").string(), "A readme to store!");

  elfshaker::store( elfshaker_data_dir, worktree_path, { "README.md" }, "myrevision-hash"); 

  {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, "init", elfshaker::ExtractOptions{
      .verify = false,
      .force = true,
      .reset = false,
      .num_workers = 32
    });

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }
  {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, "myrevision-hash", elfshaker::ExtractOptions{
      .verify = false,
      .force = true,
      .reset = false,
      .num_workers = 32
    });

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }

    {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, "init", elfshaker::ExtractOptions{
      .verify = true,
      .force = true,
      .reset = false,
      .num_workers = 32
    });

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }
  {
    elfshaker::pack(elfshaker_data_dir, worktree_path, "mypack", 12, 0);
  }

  {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, "myrevision-hash", elfshaker::ExtractOptions{
      .verify = false,
      .force = true,
      .reset = false,
      .num_workers = 32
    });

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }


  {
    auto status_list = elfshaker::status(elfshaker_data_dir, worktree_path, "myrevision-hash");
    std::cout << "Status List : " << std::endl;
    for (auto st : status_list) {
      std::cout << st << std::endl;
    }
  }
}