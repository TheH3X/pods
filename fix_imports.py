import os
import glob

def fix_missing_traits():
    for root, dirs, files in os.walk('src'):
        for file in files:
            if not file.endswith('.rs'): continue
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
            
            modified = False
            
            # Subclass missing imports
            if 'use glib::subclass::prelude::*;' in content and 'use gtk::subclass::prelude::*;' not in content:
                content = content.replace('use glib::subclass::prelude::*;', 'use glib::subclass::prelude::*;\nuse gtk::subclass::prelude::*;')
                modified = True
            
            # GIO subclass list models
            if 'ListModelImpl' in content and 'use gio::subclass::prelude::*;' not in content and 'mod imp' in content:
                content = content.replace('use glib::subclass::prelude::*;', 'use glib::subclass::prelude::*;\nuse gio::subclass::prelude::*;')
                modified = True
                
            if modified:
                with open(path, 'w') as f:
                    f.write(content)

fix_missing_traits()
