---
sidebar_position: 1
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import { RateLimits } from '@site/src/components/RateLimits';
import { OnboardingProviderSetup } from '@site/src/components/OnboardingProviderSetup';
import { ModelSelectionTip } from '@site/src/components/ModelSelectionTip';
import MacDesktopInstallButtons from '@site/src/components/MacDesktopInstallButtons';
import WindowsDesktopInstallButtons from '@site/src/components/WindowsDesktopInstallButtons';
import LinuxDesktopInstallButtons from '@site/src/components/LinuxDesktopInstallButtons';
import { PanelLeft } from 'lucide-react';

# Install Goose

<Tabs>
  <TabItem value="mac" label="macOS" default>
    Choose to install the Desktop and/or CLI version of Goose:

    <Tabs groupId="interface">
      <TabItem value="ui" label="goose Desktop" default>
        Install Goose Desktop directly from the browser or with [Homebrew](https://brew.sh/).

        <h3 style={{ marginTop: '1rem' }}>Option 1: Install via Download</h3>
        <MacDesktopInstallButtons/>

        <div style={{ marginTop: '1rem' }}>
          1. Unzip the downloaded zip file.
          2. Run the executable file to launch the Goose Desktop application.

          :::tip Updating Goose
          It's best to keep Goose updated by periodically running the installation steps again.
          :::
        </div>
        <h3>Option 2: Install via Homebrew</h3>
        Homebrew downloads the [same app](https://github.com/Homebrew/homebrew-cask/blob/master/Casks/b/block-goose.rb) but can take care of updates too.
        ```bash
          brew install --cask block-goose
        ```
        ---
        <div style={{ marginTop: '1rem' }}>
          :::info Permissions
          If you're on an Apple Mac M3 and the Goose Desktop app shows no window on launch, check and update the following:

          Ensure the `~/.config` directory has read and write access.

          Goose needs this access to create the log directory and file. Once permissions are granted, the app should load correctly. For steps on how to do this, refer to the  [Troubleshooting Guide](/docs/troubleshooting.md#macos-permission-issues)
          :::
        </div>
      </TabItem>
      <TabItem value="cli" label="goose CLI">
        Install Goose directly from the browser or with [Homebrew](https://brew.sh/).

        <h3 style={{ marginTop: '1rem' }}>Option 1: Install via Download script</h3>
        Run the following command to install the latest version of Goose on macOS:

        ```sh
        curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash
        ```
        This script will fetch the latest version of Goose and set it up on your system.

        If you'd like to install without interactive configuration, disable `CONFIGURE`:

        ```sh
        curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash
        ```

        :::tip Updating Goose
        It's best to keep Goose updated. To update Goose, run:
        ```sh
        goose update
        ```
        :::

        <h3>Option 2: Install via Homebrew</h3>
        Homebrew downloads the [a precompiled CLI tool](https://github.com/Homebrew/homebrew-core/blob/master/Formula/b/block-goose-cli.rb) and can take care of updates.
        ```bash
        brew install block-goose-cli
        ```
      </TabItem>
    </Tabs>
  </TabItem>

  <TabItem value="linux" label="Linux">
    Choose to install the Desktop and/or CLI version of Goose:

    <Tabs groupId="interface">
      <TabItem value="ui" label="goose Desktop" default>
        Install Goose Desktop directly from the browser.

        <h3 style={{ marginTop: '1rem' }}>Install via Download</h3>
        <LinuxDesktopInstallButtons/>

        <div style={{ marginTop: '1rem' }}>
          **For Debian/Ubuntu-based distributions:**
          1. Download the DEB file
          2. Navigate to the directory where it is saved in a terminal
          3. Run `sudo dpkg -i (filename).deb`
          4. Launch Goose from the app menu

          :::tip Updating Goose
          It's best to keep Goose updated by periodically running the installation steps again.
          :::
        </div>
      </TabItem>
      <TabItem value="cli" label="goose CLI">
        Run the following command to install the Goose CLI on Linux:

        ```sh
        curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash
        ```
        This script will fetch the latest version of Goose and set it up on your system.

        If you'd like to install without interactive configuration, disable `CONFIGURE`:

        ```sh
        curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash
        ```

        :::tip Updating Goose
        It's best to keep Goose updated. To update Goose, run:
        ```sh
        goose update
        ```
        :::
      </TabItem>
    </Tabs>
  </TabItem>

  <TabItem value="windows" label="Windows">
    Choose to install the Desktop and/or CLI version of Goose:

    <Tabs groupId="interface">
      <TabItem value="ui" label="goose Desktop" default>
        Install Goose Desktop directly from the browser.

        <h3 style={{ marginTop: '1rem' }}>Install via Download</h3>
        <WindowsDesktopInstallButtons/>

        <div style={{ marginTop: '1rem' }}>
          1. Unzip the downloaded zip file.
          2. Run the executable file to launch the Goose Desktop application.

          :::tip Updating Goose
          It's best to keep Goose updated by periodically running the installation steps again.
          :::
        </div>
      </TabItem>
      <TabItem value="cli" label="goose CLI">
        To install Goose natively on Windows, you need one of the following environments:
        - **Git Bash** (recommended): Comes with [Git for Windows](https://git-scm.com/download/win)
        - **MSYS2**: Available from [msys2.org](https://www.msys2.org/)
        - **PowerShell**: Available on Windows 10/11 by default

        Run the installation command in your chosen environment:

        ```bash
        curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash
        ```

        To install without interactive configuration, disable `CONFIGURE`:

        ```bash
        curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash
        ```

        **PowerShell Installation:**
        Download the PowerShell installation script to your current directory.

        ```powershell
        Invoke-WebRequest -Uri "https://raw.githubusercontent.com/block/goose/main/download_cli.ps1" -OutFile "download_cli.ps1";
        ```
        Then run the script to install Goose:
        ```powershell
        .\download_cli.ps1
        ```

        :::info Windows PATH Setup
        If you see a warning that Goose is not in your PATH, you need to add Goose to your PATH:

        <details>
          <summary>For Git Bash/MSYS2</summary>
          ```bash
          echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
          source ~/.bashrc
          ```
        </details>

        <details>
          <summary>For PowerShell</summary>
          ```powershell
          # Add to your PowerShell profile
          $profilePath = $PROFILE
          if (!(Test-Path $profilePath)) { New-Item -Path $profilePath -ItemType File -Force }
          Add-Content -Path $profilePath -Value '$env:PATH = "$env:USERPROFILE\.local\bin;$env:PATH"'
          # Reload profile or restart PowerShell
          . $PROFILE
          ```
        </details>

        After updating your PATH, you can run `goose` commands from any directory.
        :::

        <details>
        <summary>Install via Windows Subsystem for Linux (WSL)</summary>

          We recommend running the Goose CLI natively on Windows, but you can use WSL if you prefer a Linux-like environment.

          1. Open [PowerShell](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows) as Administrator and install WSL and the default Ubuntu distribution:

          ```bash
          wsl --install
          ```

          2. If prompted, restart your computer to complete the WSL installation. Once restarted, or if WSL is already installed, launch your Ubuntu shell by running:

          ```bash
          wsl -d Ubuntu
          ```

          3. Run the Goose installation script:
          ```bash
          curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash
          ```
          :::tip
            If you encounter any issues on download, you might need to install `bzip2` to extract the downloaded file:

            ```bash
            sudo apt update && sudo apt install bzip2 -y
            ```
          :::

          If you'd like to install without interactive configuration, disable `CONFIGURE`:

          ```sh
          curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash
          ```

          If needed, add Goose to your path:

          ```
          echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
          echo 'export OPENAI_API_KEY=your_api_key' >> ~/.bashrc
          source ~/.bashrc
          ```

        </details>
      </TabItem>
    </Tabs>
  </TabItem>
</Tabs>

## Set LLM Provider
Goose works with [supported LLM providers][providers] that give Goose the AI intelligence it needs to understand your requests. On first use, you'll be prompted to configure your preferred provider.

<Tabs groupId="interface">
  <TabItem value="ui" label="goose Desktop" default>
    On the welcome screen, choose how to configure a provider:
    <OnboardingProviderSetup />
  </TabItem>
  <TabItem value="cli" label="goose CLI">
    The CLI automatically enters configuration mode where you can choose how to configure a provider:

    <OnboardingProviderSetup />

    Example configuration flow:

    ```
    ┌   goose-configure
    │
    ◇ How would you like to set up your provider?
    │ Tetrate Agent Router Service Login
    │
    Opening browser for Tetrate Agent Router Service authentication...
    [Goose opens the browser and prints details]

    Authentication complete!

    Configuring Tetrate Agent Router Service...
    ✓ Tetrate Agent Router Service configuration complete
    ✓ Models configured successfully

    Testing configuration...
    ✓ Configuration test passed!
    ✓ Developer extension enabled!
    └ Tetrate Agent Router Service setup complete! You can now use Goose.
  ```

  :::info Windows Users
  If you choose to manually configure a provider, when prompted during configuration, choose to not store to keyring. If you encounter keyring errors when setting API keys, you can set environment variables manually instead:

  ```bash
  export OPENAI_API_KEY={your_api_key}
  ```

  Then run `goose configure` again. Goose will detect the environment variable and display:

  ```
  ● OPENAI_API_KEY is set via environment variable
  ```

  To make API keys persist across sessions, add them to your shell profile:
  ```bash
  echo 'export OPENAI_API_KEY=your_api_key' >> ~/.bashrc
  source ~/.bashrc
  ```
  :::
  </TabItem>
</Tabs>

:::tip
<ModelSelectionTip />
:::

## Update Provider
You can change your LLM provider and/or model or update your API key at any time.

<Tabs groupId="interface">
  <TabItem value="ui" label="goose Desktop" default>
    1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar.
    2. Click the `Settings` button on the sidebar.
    3. Click the `Models` tab.
    4. Choose to update your provider, switch models, or click `Reset Provider and Model` to clear your settings and return to the welcome screen. See details about these [configuration options](/docs/getting-started/providers#configure-provider).
  </TabItem>
  <TabItem value="cli" label="goose CLI">
    1. Run the following command:
    ```sh
    goose configure
    ```
    2. Select `Configure Providers` from the menu.
    3. Follow the prompts to choose your LLM provider and enter or update your API key.

    **Example:**

    To select an option during configuration, use the up and down arrows to highlight your choice then press Enter.

    ```
    ┌   goose-configure
    │
    ◇ What would you like to configure?
    │ Configure Providers
    │
    ◇ Which model provider should we use?
    │ Google Gemini
    │
    ◇ Provider Google Gemini requires GOOGLE_API_KEY, please enter a value
    │▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪
    │
    ◇ Enter a model from that provider:
    │ gemini-2.0-flash-exp
    │
    ◇  Hello there! You're all set to use me, so please ask away!
    │
    └  Configuration saved successfully
    ```
  </TabItem>
</Tabs>

<RateLimits />

## Running Goose

<Tabs groupId="interface">
    <TabItem value="ui" label="goose Desktop" default>
        Starting a session in the Goose Desktop is straightforward. After choosing your provider, you'll see the session interface ready for use.

        Type your questions, tasks, or instructions directly into the input field, and Goose will get to work immediately.
    </TabItem>
    <TabItem value="cli" label="goose CLI">
        From your terminal, navigate to the directory you'd like to start from and run:
        ```sh
        goose session
        ```
    </TabItem>
</Tabs>

## Shared Configuration Settings

The Goose CLI and Desktop UI share all core configurations, including LLM provider settings, model selection, and extension configurations. When you install or configure extensions in either interface, the settings are stored in a central location at `~/.config/goose/config.yaml`, making them available to both the Desktop application and CLI. This makes it convenient to switch between interfaces while maintaining consistent settings.

:::info
While core configurations are shared between interfaces, extensions have flexibility in how they store authentication credentials. Some extensions may use the shared config file while others implement their own storage methods.
:::

<Tabs groupId="interface">
    <TabItem value="ui" label="goose Desktop" default>
        Navigate to shared configurations through:
        1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar.
        2. Click the `Settings` button on the sidebar.
    </TabItem>
    <TabItem value="cli" label="goose CLI">
        Use the following command to manage shared configurations:
        ```sh
        goose configure
        ```
    </TabItem>
</Tabs>

## Additional Resources

You can also configure Extensions to extend Goose's functionality, including adding new ones or toggling them on and off. For detailed instructions, visit the [Using Extensions Guide][using-extensions].

[using-extensions]: /docs/getting-started/using-extensions
[providers]: /docs/getting-started/providers
[handling-rate-limits]: /docs/guides/handling-llm-rate-limits-with-goose
[mcp]: https://www.anthropic.com/news/model-context-protocol
