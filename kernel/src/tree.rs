use crate::ResourceCapabilityArena;

unsafe fn skew(mut t: *mut ResourceCapabilityArena) -> *mut ResourceCapabilityArena {
    if t.is_null() { return t; }
    
    unsafe {
        if (*t).left.is_null() {
            return t;
        }

        if (*(*t).left).level == (*t).level {
            let l = (*t).left;
            (*t).left = (*l).right;
            (*l).right = t;
            t = l;
        }
    }
    t
}

unsafe fn split(mut t: *mut ResourceCapabilityArena) -> *mut ResourceCapabilityArena {
    if t.is_null() { return t; }
    
    unsafe {
        if (*t).right.is_null() || (*(*t).right).right.is_null() {
            return t;
        }

        if (*t).level == (*(*(*t).right).right).level {
            let r = (*t).right;
            (*t).right = (*r).left;
            (*r).left = t;
            t = r;
            (*t).level += 1;
        }
    }
    t
}

pub unsafe fn insert_arena(
    mut root: *mut ResourceCapabilityArena, 
    node: *mut ResourceCapabilityArena
) -> *mut ResourceCapabilityArena {
    unsafe {
        if root.is_null() {
            (*node).left = core::ptr::null_mut();
            (*node).right = core::ptr::null_mut();
            (*node).level = 1;
            return node;
        }

        if (*node).arenaid < (*root).arenaid {
            (*root).left = insert_arena((*root).left, node);
        } else if (*node).arenaid > (*root).arenaid {
            (*root).right = insert_arena((*root).right, node);
        } else {
            // ID collision handling (highly unlikely with secure random IDs)
            return root;
        }

        // Balance the node on the way back up
        root = skew(root);
        root = split(root);
    }
    
    root
}

pub unsafe fn find_arena(
    root: *mut ResourceCapabilityArena, 
    target_id: u64
) -> *mut ResourceCapabilityArena {
    let mut current = root;
    
    unsafe {
        while !current.is_null() {
            if target_id == (*current).arenaid {
                return current;
            } else if target_id < (*current).arenaid {
                current = (*current).left;
            } else {
                current = (*current).right;
            }
        }
    }
    
    core::ptr::null_mut() // Not found
}

pub unsafe fn delete_arena(
    mut t: *mut ResourceCapabilityArena, 
    target_id: u64
) -> *mut ResourceCapabilityArena {
    if t.is_null() { return t; }

    unsafe {

        if target_id < (*t).arenaid {
            (*t).left = delete_arena((*t).left, target_id);
        } else if target_id > (*t).arenaid {
            (*t).right = delete_arena((*t).right, target_id);
        } else {
            // Found the node to destroy
            if (*t).left.is_null() {
                return (*t).right;
            } else if (*t).right.is_null() {
                return (*t).left;
            } else {
                // Two children: Get successor
                let mut succ = (*t).right;
                while !(*succ).left.is_null() {
                    succ = (*succ).left;
                }
                // Swap tracking keys
                (*t).arenaid = (*succ).arenaid;
                // Recursively delete the successor
                (*t).right = delete_arena((*t).right, (*succ).arenaid);
            }
        }

        // Rebalance levels
        if let Some(balanced_t) = update_level(t) {
            t = balanced_t;
        }
    }
        
    t
}

unsafe fn update_level(mut t: *mut ResourceCapabilityArena) -> Option<*mut ResourceCapabilityArena> {
    unsafe {
        let left_lvl = if (*t).left.is_null() { 0 } else { (*(*t).left).level };
        let right_lvl = if (*t).right.is_null() { 0 } else { (*(*t).right).level };

        if left_lvl < (*t).level - 1 || right_lvl < (*t).level - 1 {
            (*t).level -= 1;
            if !(*t).right.is_null() && (*(*t).right).level > (*t).level {
                (*(*t).right).level = (*t).level;
            }
            t = skew(t);
            if !(*t).right.is_null() {
                (*t).right = skew((*t).right);
                if !(*(*t).right).right.is_null() {
                    (*(*t).right).right = skew((*(*t).right).right);
                }
            }
            t = split(t);
            if !(*t).right.is_null() {
                (*t).right = split((*t).right);
            }
            return Some(t);
        }
    }
    None
}
