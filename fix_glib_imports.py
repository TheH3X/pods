import os
import re

def fix():
    for root, dirs, files in os.walk('src'):
        for file in files:
            if not file.endswith('.rs'): continue
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
            
            new_content = content
            new_content = re.sub(r'\buse glib::Properties;', 'use gtk::glib::Properties;', new_content)
            new_content = re.sub(r'\buse glib::prelude::\*', 'use gtk::glib::prelude::\*', new_content)
            new_content = re.sub(r'\buse glib::subclass::prelude::\*', 'use gtk::glib::subclass::prelude::\*', new_content)
            new_content = re.sub(r'\buse adw::subclass::prelude::\*', 'use gtk::glib::subclass::prelude::\*;\nuse adw::subclass::prelude::\*', new_content)
            
            if new_content != content:
                with open(path, 'w') as f:
                    f.write(new_content)

fix()
