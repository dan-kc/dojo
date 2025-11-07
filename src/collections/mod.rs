// Vec operation patterns  
pub mod merge_sorted_vectors;
pub mod chunk_vector;
pub mod dedup_preserve_order;
pub mod drain_and_sum;
pub mod group_by;
pub mod partition_vector;
pub mod reverse_chunks;
pub mod rotate_vector;
pub mod sliding_windows;
pub mod splice_replace;
pub mod vec_tracker;
pub mod zip_vectors;

// HashMap operation patterns
pub mod count_char_frequencies;
pub mod merge_hashmaps;
pub mod hashmap_transform;
pub mod intersect_hashmaps;
pub mod memoizer;
pub mod word_index;

// HashSet operation patterns
pub mod union_all_sets;
pub mod intersect_all_sets;
pub mod find_unique_elements;
pub mod partition_set;
pub mod spell_check;
pub mod find_connected_components;
pub mod analyze_duplicates;
pub mod jaccard_similarity;
pub mod find_mutual_friends;
pub mod difference_chain;

// BTree operation patterns
pub mod btree_range_query;
pub mod btree_split_map;
pub mod btree_k_extremes;
pub mod sliding_window_tracker;
pub mod btree_merge_maps;
pub mod btree_consecutive_ranges;
pub mod btree_timeline;
pub mod btree_custom_ordering;
pub mod btree_predecessor_successor;
pub mod btree_simple_index;
pub mod btree_stepped_union;

// Specialized collection patterns
pub mod palindrome_checker;
pub mod deque_rotation;
pub mod sliding_window_maximum;
pub mod task_scheduler;
pub mod median_tracker;
pub mod simple_lru_cache;
pub mod card_deck;
pub mod work_stealing_deque;
