<script lang="ts">
  import Tooltip from './Tooltip.svelte';

  export let src: string;
  export let name: string;
  export let tooltipText: string;
  export let width: number | undefined = undefined;
  export let height: number | undefined = undefined;

  let image: HTMLImageElement | null = null;

  let tooltipOpen = false;

  function showTooltip() {
    tooltipOpen = true;
  }

  function hideTooltip() {
    tooltipOpen = false;
  }
</script>

<img
  bind:this={image}
  src={`/images/${src}`}
  alt=""
  draggable={false}
  {width}
  {height}
  on:mouseenter={showTooltip}
  on:mouseleave={hideTooltip}
/>

<Tooltip
  open={tooltipOpen}
  anchor={image}
  title={name}
  text={tooltipText}
/>

<style>
  img {
    display: block;
    object-fit: cover;
    border-radius: inherit;
  }
</style>
