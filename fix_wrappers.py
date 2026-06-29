import os
import re

def fix_wrappers():
    for root, dirs, files in os.walk('src'):
        for file in files:
            if not file.endswith('.rs'): continue
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
            
            # This is a bit tricky, but basically we want to find glib::wrapper! blocks.
            # Usually they look like:
            # glib::wrapper! {
            #     pub(crate) struct ComposeEnvRow(ObjectSubclass<imp::ComposeEnvRow>)
            #         @extends gtk::ListBoxRow, gtk::Widget;
            # }
            # If they miss @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, we can just replace the semicolon.
            
            def replacer(m):
                body = m.group(1)
                extends = body
                if '@implements' not in body:
                    if 'gtk::ListBoxRow' in body:
                        return body.replace(';', ',\n        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;')
                    elif 'gtk::Widget' in body or 'adw::NavigationPage' in body or 'adw::PreferencesPage' in body or 'gtk::Window' in body or 'gtk::ApplicationWindow' in body:
                        return body.replace(';', ',\n        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;')
                return body
            
            new_content = re.sub(r'glib::wrapper!\s*\{([^}]*)\}', lambda m: 'glib::wrapper! {' + replacer(m) + '}', content)
            
            if new_content != content:
                with open(path, 'w') as f:
                    f.write(new_content)

fix_wrappers()
