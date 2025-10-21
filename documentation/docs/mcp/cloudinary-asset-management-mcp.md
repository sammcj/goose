---
title: Cloudinary Asset Management Extension
description: Add Cloudinary Asset Management MCP Server as a Goose Extension
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import GooseDesktopInstaller from '@site/src/components/GooseDesktopInstaller';
import YouTubeShortEmbed from '@site/src/components/YouTubeShortEmbed';

<YouTubeShortEmbed videoUrl="https://www.youtube.com/embed/4diEvoRFVrQ" />

This tutorial covers how to add the [Cloudinary Asset Management MCP Server](https://github.com/cloudinary/asset-management-js) as a Goose extension to automate complex image processing workflows that would typically require specialized design software or manual editing.

:::tip TLDR

<Tabs groupId="interface">
  <TabItem value="ui" label="goose Desktop" default>
  [Launch the installer](goose://extension?cmd=npx&arg=-y&arg=--package&arg=@cloudinary/asset-management&arg=--&arg=mcp&arg=start&id=cloudinary&name=Cloudinary%20Asset%20Management&description=Powerful%20media%20processing%20and%20transformation&env=CLOUDINARY_URL%3DCloudinary%20URL)
  </TabItem>
  <TabItem value="cli" label="goose CLI">
  **Command**
  ```sh
  npx -y --package @cloudinary/asset-management -- mcp start
  ```
  </TabItem>
</Tabs>
  **Environment Variable**
  ```
  CLOUDINARY_URL: cloudinary://<your_api_key>:<your_api_secret>@<your_cloud_name>
  ```
:::

## Configuration

:::info
Note that you'll need [Node.js](https://nodejs.org/) installed on your system to run this command, as it uses `npx`. You'll also need a [Cloudinary account](https://cloudinary.com/users/register/free).
:::

<Tabs groupId="interface">
  <TabItem value="ui" label="goose Desktop" default>
  <GooseDesktopInstaller
    extensionId="cloudinary"
    extensionName="Cloudinary Asset Management"
    description="Powerful media processing and transformation"
    command="npx"
    args={["-y", "--package", "@cloudinary/asset-management", "--", "mcp", "start"]}
    envVars={[{ name: "CLOUDINARY_URL", label: "Cloudinary URL (cloudinary://api_key:api_secret@cloud_name)" }]}
    apiKeyLink="https://console.cloudinary.com/settings/api-keys"
    apiKeyLinkText="Get your Cloudinary URL"
  />
  </TabItem>
  <TabItem value="cli" label="goose CLI">
  1. Run the `configure` command:
  ```sh
  goose configure
  ```

  2. Choose to add a `Command-line Extension`
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◆  What type of extension would you like to add?
    │  ○ Built-in Extension 
    // highlight-start    
    │  ● Command-line Extension (Run a local command or script)
    // highlight-end    
    │  ○ Remote Extension (SSE) 
    │  ○ Remote Extension (Streaming HTTP) 
    └ 
  ```

  3. Give your extension a name
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What type of extension would you like to add?
    │  Command-line Extension 
    │
    // highlight-start
    ◆  What would you like to call this extension?
    │  cloudinary
    // highlight-end
    └ 
  ```

  4. Enter the command
  ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What type of extension would you like to add?
    │  Command-line Extension 
    │
    ◇  What would you like to call this extension?
    │  cloudinary
    │
    // highlight-start
    ◆  What command should be run?
    │  npx -y --package @cloudinary/asset-management -- mcp start
    // highlight-end
    └ 
  ```  

  5. Enter the number of seconds Goose should wait for actions to complete before timing out. Default is 300s
   ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What type of extension would you like to add?
    │  Command-line Extension 
    │
    ◇  What would you like to call this extension?
    │  cloudinary
    │
    ◇  What command should be run?
    │  npx -y --package @cloudinary/asset-management -- mcp start
    │
    // highlight-start
    ◆  Please set the timeout for this tool (in secs):
    │  300
    // highlight-end
    └ 
  ```  

  6. Choose to add a description. If you select "Yes" here, you will be prompted to enter a description for the extension.
   ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What type of extension would you like to add?
    │  Command-line Extension 
    │
    ◇  What would you like to call this extension?
    │  cloudinary
    │
    ◇  What command should be run?
    │  npx -y --package @cloudinary/asset-management -- mcp start
    │
    ◆  Please set the timeout for this tool (in secs):
    │  300
    │
    // highlight-start
    ◇  Would you like to add a description?
    │  No
    // highlight-end
    │
    └ 
  ```

  7. Obtain your [Cloudinary URL](https://console.cloudinary.com/settings/api-keys) from your dashboard and paste it in.

   ```sh
    ┌   goose-configure 
    │
    ◇  What would you like to configure?
    │  Add Extension (Connect to a new extension) 
    │
    ◇  What type of extension would you like to add?
    │  Command-line Extension 
    │
    ◇  What would you like to call this extension?
    │  cloudinary
    │
    ◇  What command should be run?
    │  npx -y --package @cloudinary/asset-management -- mcp start
    │
    ◇  Please set the timeout for this tool (in secs):
    │  300
    │
    ◇  Would you like to add a description?
    │  No
    │
    // highlight-start
    ◆  Would you like to add environment variables?
    │  Yes 
    │
    ◇  Environment variable name:
    │  CLOUDINARY_URL
    │
    ◇  Environment variable value:
    │  cloudinary://<your_api_key>:<your_api_secret>@<your_cloud_name>
    │
    ◇  Add another environment variable?
    │  No 
    // highlight-end
    └  Added cloudinary extension
  ```  

  </TabItem>
</Tabs>

## Example Usage

Let's use the Cloudinary extension to find and transform product images with advanced visual effects. In this example, we'll demonstrate how Goose can help you:

1. Search for specific images in your media library
2. Apply complex transformations including background removal
3. Add text overlays with precise positioning

:::info LLM
Anthropic's Claude 4 Sonnet was used for this task.
:::

### Goose Prompt
```
1. find shoe images in my Cloudinary samples that have 'shoe' in the filename or public ID.
2. Let's transform the first result and only extract just the shoes
3. Add a white text overlay that says I love shoes. Make sure it's above the shoes
4. Add a background to the image. It should be black
```

### Goose Output
```
I'll help you find shoe images in your Cloudinary samples and apply the transformations you've requested. Let me start by searching for images with 'shoe' in the filename or public ID.

─── list-images | cloudinaryassetmanagement ──────────────────────────
maxResults: 50
prefix: samples
type: upload

Perfect! I found the shoe images in your Cloudinary samples. I can see:

1. **`samples/shoe`** - A single shoe image (publicId: `samples/shoe`)
2. **`samples/ecommerce/shoes`** - A pair of shoes (publicId: `samples/ecommerce/shoes`)

Let me use the first result (`samples/shoe`) as requested and apply all the transformations you want:

1. Extract just the shoes (using background removal)
2. Add white text overlay "I love shoes" above the shoes
3. Add a black background

Here's the transformed image URL with all your requested modifications:

https://res.cloudinary.com/dd8qwbjtv/image/upload/e_background_removal/b_black/l_text:Arial_60:I%20love%20shoes,co_white,g_north,y_50/samples/shoe

Let me break down what each transformation does:

- `e_background_removal` - Removes the background and extracts just the shoe
- `b_black` - Adds a black background to replace the removed background
- `l_text:Arial_60:I%20love%20shoes,co_white,g_north,y_50` - Adds white text overlay:
  - `Arial_60` - Arial font, size 60
  - `I%20love%20shoes` - The text (URL encoded)
  - `co_white` - White color
  - `g_north` - Positioned at the top (north)
  - `y_50` - 50 pixels offset from the top

Here are the images we found with "shoe" in the filename/public ID:

1. **First result (used for transformation)**: `samples/shoe`
   - Original: https://res.cloudinary.com/dd8qwbjtv/image/upload/v1708364167/samples/shoe.jpg
   - **Transformed**: https://res.cloudinary.com/dd8qwbjtv/image/upload/e_background_removal/b_black/l_text:Arial_60:I%20love%20shoes,co_white,g_north,y_50/samples/shoe

2. **Second result**: `samples/ecommerce/shoes`
   - Original: https://res.cloudinary.com/dd8qwbjtv/image/upload/v1708364144/samples/ecommerce/shoes.png

The transformed image now has:
✅ Just the shoes extracted (background removed)
✅ White text "I love shoes" positioned above the shoes
✅ Black background applied

You can click on the transformed URL to see the final result!
```


