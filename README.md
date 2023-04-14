# Resident Evil 4 (2023) Treasure Optimizer

This is a tool for Resident Evil 4 allowing you to increase the profit you get for selling treasures by optimally allocating gems in a way that maximizes the value for each treasure you own.

## Installation

Go to the [Releases page](https://github.com/agrear/re4-treasure-optimizer/releases) and download either the installer or the portable executable (works only if you have the necessary [dependencies to run a Tauri app](https://tauri.app/v1/guides/building/windows#skipping-installation) installed).

## How To

Select the available gems and treasures by left clicking to add or right clicking to remove. Once you click the 'Optimize' button the treasures and their allocated gems are shown below. You can hover over each result to get details about the total value and bonus.

If you only want to sell fully socketed treasures disable the 'Allow empty sockets' option. Additionally you can also optimize for maximum bonus (multiplier) rather than value by selecting a different objective function.

## How It Works

The tool uses Dynamic Programming to efficiently calculate all possible allocations of gems to treasures. However, keep in mind that this is still an exponentially scaling problem; therefore using more than a sum of about 30 gems and treasures will drastically increase computation time and memory cost.

## Disclaimer

All images pertaining to gems and treasures belong to CAPCOM Co., Ltd.
