#include <elfshaker-cxxbridge/lib.h>
#include <rust/cxx.h>
#include <vector>
#include <iostream>
int main(int argc, char** argv) {
  std::vector<uint64_t> input = { 4, 5, 6};
  rust::Slice<const ::std::uint64_t> slice{input.data(), input.size()};
  bridge::print(slice);

  std::string worktree_path = argv[1];
  worktree_path += "/worktree";

  std::string elfshaker_data_dir = argv[1];
  elfshaker_data_dir += "/elfshaker_data";

  try {
    bridge::init_elfshaker_store( elfshaker_data_dir, worktree_path);
  } catch(const rust::Error &e) {
    std::cout << e.what() << std::endl;
  }

  int i;
  std::cin >> i;
  //std::cout << "Initialization " << std::boolalpha << result << std::endl;
  bridge::store( elfshaker_data_dir, worktree_path, { std::string{argv[2]} }, "banana"); 

  {
    auto extracted = bridge::extract( elfshaker_data_dir, worktree_path, "init", bridge::ExtractOptions{
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
std::cin >> i;
  {
    auto extracted = bridge::extract( elfshaker_data_dir, worktree_path, "banana", bridge::ExtractOptions{
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

  std::cin >> i;
    {
    auto extracted = bridge::extract( elfshaker_data_dir, worktree_path, "init", bridge::ExtractOptions{
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
std::cin >> i;
  {
    bridge::pack(elfshaker_data_dir, worktree_path, "mypack", 12, 0);
  }

std::cin >> i;
  {
    auto extracted = bridge::extract( elfshaker_data_dir, worktree_path, "banana", bridge::ExtractOptions{
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


std::cin >> i;
  {
    auto status_list = bridge::status(elfshaker_data_dir, worktree_path, "banana");
    std::cout << "Status List : " << std::endl;
    for (auto st : status_list) {
      std::cout << st << std::endl;
    }
  }
  return 0;
}