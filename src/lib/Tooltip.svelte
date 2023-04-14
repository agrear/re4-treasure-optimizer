<script lang="ts">
  import { computePosition, flip, offset, shift } from '@floating-ui/dom';

  export let title: string;
  export let text: string;
  export let anchor: HTMLElement | null | undefined = undefined;
  export let open: boolean = false;
  export let width: number = 200;

  let tooltip: HTMLDivElement | null = null;

  $: if (open && anchor) {
    updatePosition();
  }

  function updatePosition() {
    if (anchor === null || anchor === undefined || tooltip === null) {
      return;
    }

    computePosition(anchor, tooltip, {
      placement: 'right-start',
      middleware: [
        offset(8),
        flip(),
        shift({ padding: 8 })
      ]
    }).then(({ x, y }) => {
      Object.assign(tooltip.style, {
        left: `${x}px`,
        top: `${y}px`
      });
    });
  }
</script>

<div
  bind:this={tooltip}
  class="tooltip"
  style="width: {width}px; display: {open ? 'block' : 'none'};"
>
  <div class="title">{title}</div>
  <div class="body">{text}</div>
</div>

<style>
  .tooltip {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 99;
    background: #222;
    text-align: start;
    color: white;
    font-weight: bold;
    line-height: normal;
    padding: 8px;
    border: 1px solid #c6c6c6;
    border-radius: 8px;
  }

  .title {
    margin-bottom: 8px;
    font-size: 110%;
  }

  .body {
    font-size: 90%;
    white-space: pre-line;
  }
</style>
