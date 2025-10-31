use maud::{Markup, html};

pub fn terminal_styles() -> Markup {
    html! {
        style {
            r#"
            :root {
                --bg-primary: #1a1d29;
                --bg-secondary: #232530;
                --bg-tertiary: #2a2d3a;
                --text-primary: #e4e4e7;
                --text-secondary: #a1a1aa;
                --text-muted: #71717a;
                --accent: #14b8a6;
                --accent-hover: #0d9488;
                --border: #374151;
                --code-bg: #111827;
                --prompt: #10b981;
            }

            * {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
            }

            body {
                background: var(--bg-primary);
                color: var(--text-primary);
                font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
                line-height: 1.6;
                min-height: 100vh;
                padding: 2rem;
                font-size: 16px;
                display: flex;
                flex-direction: column;
            }

            .container {
                max-width: 1400px;
                margin: 0 auto;
                flex: 1;
                display: flex;
                flex-direction: column;
                width: 90%;
            }

            .main-content {
                flex: 1;
            }

            .header-container {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 2rem;
                flex-wrap: wrap;
            }

            .header-left {
                flex: 1;
            }

            .header-right {
                display: flex;
                flex-direction: column;
                gap: 0.3rem;
                align-items: flex-end;
                padding-top: 0.5rem;
            }

            .ascii-link {
                text-decoration: none;
                display: inline-block;
                line-height: 1;
            }

            .ascii-art {
                color: var(--text-secondary);
                margin-bottom: 0.5rem;
                font-size: 0.42rem;
                text-align: left;
                opacity: 0.8;
                margin: 0;
                padding: 0;
                transition: color 0.2s ease, text-shadow 0.2s ease;
            }

            .ascii-link:hover .ascii-art {
                color: var(--accent);
                text-shadow: 0 0 8px var(--accent), 0 0 12px var(--accent);
            }

            .subtitle {
                color: var(--accent);
                font-size: 0.75rem;
                text-align: left;
                letter-spacing: 2px;
                text-transform: uppercase;
                opacity: 0.8;
            }

            .social-link {
                color: var(--text-muted);
                text-decoration: none;
                font-size: 0.85rem;
                transition: color 0.2s ease;
            }

            .social-link:hover {
                color: var(--accent);
            }

            .bottom-prompt {
                margin: 2rem 0 1rem 0;
                font-weight: 500;
                font-size: 0.9rem;
            }

            .footer {
                text-align: center;
                padding: 2rem 0 1rem 0;
                border-top: 1px solid var(--border);
                margin-top: auto;
            }

            .source-link {
                color: var(--text-muted);
                text-decoration: none;
                font-size: 0.8rem;
                opacity: 0.8;
                transition: color 0.2s ease, opacity 0.2s ease;
            }

            .source-link:hover {
                color: var(--accent);
                opacity: 1;
            }

            .footer-text {
                margin-top: 0.5rem;
                font-size: 0.75rem;
                color: var(--text-muted);
                opacity: 0.7;
            }

            .prompt {
                margin: 1.5rem 0;
                font-weight: 500;
                font-size: 0.9rem;
            }

            .prompt-symbol {
                color: var(--prompt);
            }

            .cursor::after {
                content: '█';
                animation: blink 1.2s infinite;
                color: var(--text-primary);
                margin-left: 2px;
                opacity: 0.7;
            }

            @keyframes blink {
                0%, 50% { opacity: 1; }
                51%, 100% { opacity: 0; }
            }

            .posts-list {
                margin: 1rem 0;
            }

            .post-entry-link {
                display: block;
                text-decoration: none;
                color: inherit;
                margin: 0.8rem 0;
            }

            .post-entry {
                padding: 1rem;
                border-left: 3px solid var(--border);
                background: var(--bg-secondary);
                border-radius: 0 6px 6px 0;
                transition: all 0.2s ease;
            }

            .post-entry-link:hover .post-entry {
                border-left-color: var(--accent);
                background: var(--bg-tertiary);
                transform: translateX(4px);
            }

            .post-entry-link:hover .post-title {
                color: var(--accent);
            }

            .date {
                color: var(--text-muted);
                font-size: 0.85rem;
                font-weight: 500;
            }

            .post-title {
                color: var(--text-primary);
                font-weight: 500;
            }

            .tags {
                margin-left: 1rem;
                display: inline-flex;
                gap: 0.5rem;
            }

            .tag {
                color: var(--text-muted);
                font-size: 0.75rem;
                background: var(--bg-tertiary);
                padding: 0.2rem 0.5rem;
                border-radius: 4px;
                border: 1px solid var(--border);
            }

            .post-content {
                margin: 2rem 0;
                background: var(--bg-secondary);
                padding: 2rem;
                border-radius: 8px;
                border: 1px solid var(--border);
            }

            .post-content h1 {
                color: var(--text-primary);
                margin-bottom: 1rem;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.8rem;
                font-size: 1.8rem;
                font-weight: 600;
            }

            .post-meta {
                margin-bottom: 2rem;
                color: var(--text-muted);
                font-size: 0.9rem;
            }

            .content {
                line-height: 1.8;
            }

            .content h1, .content h2, .content h3 {
                color: var(--text-primary);
                margin: 2rem 0 1rem 0;
                font-weight: 600;
            }

            .content h2 {
                font-size: 1.4rem;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
            }

            .content h3 {
                font-size: 1.2rem;
            }

            .content p {
                margin: 1.2rem 0;
                color: var(--text-primary);
            }

            .content strong {
                color: var(--text-primary);
                font-weight: 600;
            }

            .content code {
                background: var(--code-bg);
                color: var(--accent);
                padding: 0.2rem 0.4rem;
                border-radius: 4px;
                font-size: 0.9em;
                border: 1px solid var(--border);
            }

            .content pre {
                background: var(--code-bg);
                padding: 1.5rem;
                overflow-x: auto;
                border-left: 4px solid var(--accent);
                border-radius: 6px;
                margin: 1.5rem 0;
                border: 1px solid var(--border);
            }

            .content pre code {
                background: none;
                border: none;
                padding: 0;
                color: var(--text-primary);
            }

            .content ul, .content ol {
                margin: 1rem 0;
                padding-left: 2rem;
            }

            .content li {
                margin: 0.5rem 0;
                color: var(--text-primary);
            }

            .navigation {
                margin: 1rem 0;
            }

            .navigation a {
                color: var(--text-secondary);
                text-decoration: none;
                font-weight: 500;
                transition: color 0.2s ease;
            }

            .navigation a:hover {
                color: var(--accent);
            }

            @media (max-width: 768px) {
                body {
                    padding: 1rem;
                    font-size: 13px;
                }

                .post-content {
                    padding: 1.5rem;
                }

                .container {
                    max-width: 100%;
                }
            }
            "#
        }
    }
}
