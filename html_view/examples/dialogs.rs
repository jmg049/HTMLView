//! Example demonstrating file dialogs and system notifications.
//!
//! Shows:
//! - Enabling file open/save dialogs
//! - Message dialogs (alert, confirm, prompt)
//! - System notifications
//! - JavaScript integration for dialogs
//!
//! Run with: cargo run --example dialogs

use html_view::ViewerOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Dialogs & Notifications</title>
    <style>
        body {
            font-family: system-ui, -apple-system, sans-serif;
            padding: 40px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            margin: 0;
            min-height: 100vh;
        }
        .container {
            max-width: 700px;
            margin: 0 auto;
            background: rgba(255, 255, 255, 0.1);
            padding: 30px;
            border-radius: 20px;
            backdrop-filter: blur(10px);
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
        }
        h1 {
            margin-top: 0;
            font-size: 2.5em;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
        }
        h2 {
            margin-top: 30px;
            font-size: 1.5em;
            border-bottom: 2px solid rgba(255, 255, 255, 0.3);
            padding-bottom: 10px;
        }
        .button-group {
            display: flex;
            flex-wrap: wrap;
            gap: 12px;
            margin: 20px 0;
        }
        button {
            padding: 12px 24px;
            font-size: 16px;
            cursor: pointer;
            background: white;
            color: #667eea;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            transition: all 0.3s ease;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 12px rgba(0, 0, 0, 0.2);
        }
        button:active {
            transform: translateY(0);
        }
        .description {
            background: rgba(0, 0, 0, 0.2);
            padding: 15px;
            border-radius: 8px;
            margin: 15px 0;
            line-height: 1.6;
        }
        .info {
            font-size: 14px;
            opacity: 0.9;
            margin-top: 10px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Dialogs & Notifications Example</h1>

        <div class="description">
            This example demonstrates how to enable and use various dialog types
            and system notifications in html_view applications.
        </div>

        <h2>Message Dialogs</h2>
        <div class="info">
            Standard JavaScript dialog functions (alert, confirm, prompt).
        </div>
        <div class="button-group">
            <button onclick="testAlert()">Alert Dialog</button>
            <button onclick="testConfirm()">Confirm Dialog</button>
            <button onclick="testPrompt()">Prompt Dialog</button>
        </div>

        <h2>System Notifications</h2>
        <div class="info">
            Native operating system notifications using the Notifications API.
        </div>
        <div class="button-group">
            <button onclick="showNotification()">Show Notification</button>
            <button onclick="showAdvancedNotification()">Advanced Notification</button>
        </div>

        <h2>Interactive Example</h2>
        <div class="info">
            Combining multiple dialog types in a workflow.
        </div>
        <div class="button-group">
            <button onclick="interactiveWorkflow()">Start Interactive Demo</button>
        </div>
    </div>

    <script>
        function testAlert() {
            alert('This is an alert dialog!\n\nIt displays information to the user.');
        }

        function testConfirm() {
            const result = confirm('Do you want to proceed?\n\nClick OK or Cancel.');
            if (result) {
                alert('You clicked OK!');
            } else {
                alert('You clicked Cancel.');
            }
        }

        function testPrompt() {
            const name = prompt('What is your name?', 'Guest');
            if (name !== null && name !== '') {
                alert('Hello, ' + name + '! Nice to meet you.');
            } else {
                alert('No name entered.');
            }
        }

        async function showNotification() {
            // Check if notifications are supported
            if (!('Notification' in window)) {
                alert('This browser does not support notifications');
                return;
            }

            // Check permission
            if (Notification.permission === 'granted') {
                new Notification('HTMLView Notification', {
                    body: 'This is a simple system notification!',
                    icon: 'https://via.placeholder.com/128/667eea/ffffff?text=HV',
                    tag: 'simple-notification'
                });
            } else if (Notification.permission !== 'denied') {
                const permission = await Notification.requestPermission();
                if (permission === 'granted') {
                    showNotification();
                } else {
                    alert('Notification permission denied');
                }
            } else {
                alert('Notifications have been blocked. Enable them in system settings.');
            }
        }

        async function showAdvancedNotification() {
            if (!('Notification' in window)) {
                alert('This browser does not support notifications');
                return;
            }

            if (Notification.permission === 'granted') {
                const notification = new Notification('Advanced Notification', {
                    body: 'This notification has additional features.\n\nClick it to see!',
                    icon: 'https://via.placeholder.com/128/764ba2/ffffff?text=ADV',
                    badge: 'https://via.placeholder.com/96/ffffff/764ba2?text=!',
                    tag: 'advanced-notification',
                    requireInteraction: false
                });

                notification.onclick = function() {
                    alert('You clicked the notification!');
                    notification.close();
                };
            } else if (Notification.permission !== 'denied') {
                const permission = await Notification.requestPermission();
                if (permission === 'granted') {
                    showAdvancedNotification();
                }
            }
        }

        async function interactiveWorkflow() {
            // Step 1: Confirm to start
            if (!confirm('Ready to start an interactive demo?')) {
                return;
            }

            // Step 2: Get user name
            const name = prompt('First, what\'s your name?', '');
            if (!name) {
                alert('Demo cancelled - name required.');
                return;
            }

            // Step 3: Confirm notification preference
            if (confirm('Great! Would you like to receive a notification, ' + name + '?')) {
                // Show notification if granted
                if (Notification.permission === 'granted') {
                    new Notification('Hello ' + name + '!', {
                        body: 'Thanks for trying the interactive demo!',
                        icon: 'https://via.placeholder.com/128/764ba2/ffffff?text=' + name[0].toUpperCase()
                    });
                } else if (Notification.permission !== 'denied') {
                    const permission = await Notification.requestPermission();
                    if (permission === 'granted') {
                        new Notification('Hello ' + name + '!', {
                            body: 'Thanks for trying the interactive demo!'
                        });
                    }
                }
            }

            // Step 4: Final message
            alert('Demo complete! Thanks, ' + name + '.');
        }
    </script>
</body>
</html>
    "#;

    let mut opts = ViewerOptions::inline_html(html);

    // Enable dialogs
    opts.dialog.allow_file_dialogs = true;
    opts.dialog.allow_message_dialogs = true;

    // Enable notifications
    opts.behaviour.allow_notifications = true;

    // Window settings
    opts.window.width = Some(750);
    opts.window.height = Some(700);
    opts.window.title = Some("Dialogs & Notifications Example".to_string());

    println!("Opening viewer with dialogs and notifications enabled...");
    println!("Try the buttons to test different dialog types!");
    println!();
    println!("Features enabled:");
    println!("  ✓ File dialogs");
    println!("  ✓ Message dialogs (alert, confirm, prompt)");
    println!("  ✓ System notifications");
    println!();

    html_view::open(opts)?;

    Ok(())
}
