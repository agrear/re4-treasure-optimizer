<script lang="ts">
  import { Color } from './Color';
  import { Shape } from './Shape';

  export let shape: Shape;
  export let color: Color | undefined = undefined;
  export let width: number = 12;
  export let height: number = 16;

  $: colorString = getColor(color);

  function getColor(color?: Color): string {
    switch (color) {
      case Color.Blue:
        return 'rgb(15, 82, 186)';
      case Color.Green:
        return 'rgb(0, 155, 119)';
      case Color.Purple:
        return 'rgb(89, 47, 100)';
      case Color.Red:
        //return 'rgb(161, 99, 129)';
        return 'hsl(3, 93%, 53%)';
      case Color.Yellow:
        return 'rgb(206, 164, 97)';
      default:
        return 'transparent';
    }
  }
</script>

<div
  class="socket"
  class:circle={shape === Shape.Circular}
  class:rectangle={shape === Shape.Rectangular}
  style="
    width:{width}px;
    height:{height}px;
    background-color: {colorString};
  ">
</div>

<style>
  .socket {
    border: 1px solid white;
  }

  .circle {
    border-radius: 100%;
  }

  .rectangle {
    border-radius: 20%;
  }
</style>
