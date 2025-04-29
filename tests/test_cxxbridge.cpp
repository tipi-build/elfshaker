#define BOOST_TEST_MODULE test_cxxbridge
#include <boost/test/included/unit_test.hpp>
#include <boost/test/data/test_case.hpp>
#include <boost/filesystem.hpp>

#include <elfshaker-cxxbridge/lib.h>
#include <rust/cxx.h>
#include <vector>
#include <iostream>
#include <pre/file/string.hpp>

namespace fs = boost::filesystem;


const auto TEST_DATA_SNAPSHOT_NAMES = boost::unit_test::data::make({ "myrevision.hash" , "myrevision-hash", "my revision", "myrevision_hash", "myrevision🔥hash" });
const auto TEST_DATA_PACK_NAMES = boost::unit_test::data::make({ "my-pack", "my pack", "my_pack", "my.pack", "my🔥pack" });


BOOST_DATA_TEST_CASE(store_with_separate_worktree_smoke_test, 
  TEST_DATA_SNAPSHOT_NAMES * TEST_DATA_PACK_NAMES, 
  td_snapshot_name,
  td_pack_name
) {
  auto temp_test_path = fs::temp_directory_path() / "elfshkr-test" / fs::unique_path();
  std::string worktree_path = temp_test_path.generic_string();
  worktree_path += "/worktree";

  std::string elfshaker_data_dir = temp_test_path.generic_string();
  elfshaker_data_dir += "/elfshaker_data";

  try {
    elfshaker::init_elfshaker_store( elfshaker_data_dir, worktree_path);
  } catch(const rust::Error &e) {
    std::cout << e.what() << std::endl;
  }

  pre::file::from_string((fs::path{worktree_path} / "README.md").generic_string(), "A readme to store!");

  elfshaker::store( elfshaker_data_dir, worktree_path, { "README.md" }, td_snapshot_name); 

  elfshaker::ExtractOptions extract_options{};
  extract_options.verify = false;
  extract_options.force = true;
  extract_options.reset = false;
  extract_options.num_workers = 32;

  {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, "init", extract_options);

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }
  {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, td_snapshot_name, extract_options);

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }

  {
    elfshaker::ExtractOptions extract_options_verify{};
    extract_options.verify = true;
    extract_options.force = true;
    extract_options.reset = false;
    extract_options.num_workers = 32;  

    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, "init", extract_options_verify);

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }
  {
    elfshaker::pack(elfshaker_data_dir, worktree_path, td_pack_name, 12, 0);
  }

  {
    auto extracted = elfshaker::extract( elfshaker_data_dir, worktree_path, td_snapshot_name, extract_options);

    std::cout << "A: " <<  extracted.added_file_count << "\n";
    std::cout << "D: " <<  extracted.removed_file_count << "\n";
    std::cout << "M: " <<  extracted.modified_file_count 
    << std::endl;
  }


  {
    auto status_list = elfshaker::status(elfshaker_data_dir, worktree_path, td_snapshot_name);
    std::cout << "Status List : " << std::endl;
    for (auto st : status_list) {
      std::cout << st << std::endl;
    }
  }
}