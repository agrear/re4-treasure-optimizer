<script lang="ts">
  export let value: number;
  export let min: number = 0;
  export let max: number = 99;
  export let onChange: (value: number) => void;
  export let width: number = 64;
  export let height: number = 64;

  function onClick(e: PointerEvent & {
    currentTarget: EventTarget & HTMLButtonElement;
  }) {
    if (e.button === 0) {
      onChange(Math.min(value + 1, max));
    } else if (e.button === 2) {
      onChange(Math.max(min, value - 1));
    }
  }
</script>

<div>
  <button
    style="width:{width}px; height:{height}px;"
    on:pointerdown={onClick}
  >
    <slot />
    <div class="overlay">
      <span class="count">{value}</span>
    </div>
  </button>
</div>

<style>
  button {
    position: relative;
    background-color: transparent;
    padding: 0;
  }

  button :global(img) {
    width: inherit;
    height: inherit;
    box-sizing: border-box;
    padding: 0px 2px 2px 0px;
    border-radius: inherit;
  }

  .overlay {
    position: absolute;
    bottom: 0;
    right: 0;
    pointer-events: none;
  }

  .count {
    padding: 4px;
  }
</style>
