<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { produce } from 'immer';
  import { onMount } from 'svelte';

  import { Color } from './lib/Color';
  import Image from './lib/Image.svelte'
  import NumericUpDown from './lib/NumericUpDown.svelte';
  import { ObjectiveFunction } from './lib/ObjectiveFunction';
  import { Shape } from './lib/Shape';
  import SocketComponent from './lib/Socket.svelte';

  let allGems: Gem[];
  let allTreasures: Treasure[];

  let gems: GemCollection = [];
  let treasures: TreasureCollection = [];

  let results: SocketedTreasure[] = [];

  $: filteredResults = allowEmptySockets ? results : results.filter(
    ({ sockets }) => sockets.every(({ gemId }) => gemId !== null)
  );

  $: total = filteredResults.reduce((sum, { appraisal }) => ({
    bonus: sum.bonus + appraisal.bonus,
    value: sum.value + appraisal.value
  }), { value: 0, bonus: 0 } as Appraisal);

  $: gemsEmpty = gems.reduce((sum, count) => sum + count, 0) === 0;
  $: treasuresEmpty = treasures.reduce((sum, count) => sum + count, 0) === 0;
  $: isNoneSelected = gemsEmpty && treasuresEmpty;

  let allowEmptySockets = true;
  let selectedObjectiveFunction = ObjectiveFunction.MaximizeValue;

  async function optimize() {
    results = await invoke('optimize', {
      treasures,
      gems,
      objectiveFunction: selectedObjectiveFunction
    });
  }

  function getGem(id: Id | null): Gem | undefined {
    return allGems.find(gem => gem.id === id);
  }

  function getTreasure(id: Id) {
    return allTreasures.find(treasure => treasure.id === id);
  }

  function stringifySockets(sockets: Treasure['sockets']) {
    return sockets.reduce((str, sockets, i) => (
      str + (i === 0 ? '\u03BF' : '\u25A2').repeat(sockets)
    ), '').split('').join(' ');
  }

  function stringifyGems(sockets: SocketedTreasure['sockets']) {
    const gems: Map<Id, number> = new Map();
    sockets.forEach(({ gemId }) => {
      if (gemId) {
        gems.set(gemId, (gems.get(gemId) ?? 0) + 1);
      }
    });

    const items: string[] = [];
    for (const [id, count] of gems.entries()) {
      items.push(`${count}x ${getGem(id).name}`);
    }

    return items.length === 0 ? 'None' : items.join(', ');
  }

  function formatCurrency(value: number) {
    return `${value.toLocaleString('en')} \u20A7`;
  }

  function clearResults() {
    results = [];
  }

  function resetSelection() {
    gems = produce(gems, draft => {
      draft.forEach((_, i) => draft[i] = 0);
    });

    treasures = produce(treasures, draft => {
      draft.forEach((_, i) => draft[i] = 0);
    });

    clearResults();
  }

  onMount(async () => {
    allGems = await invoke('get_gems') as Gem[];
		allTreasures = await invoke('get_treasures') as Treasure[];

    // Initialize collections to zero
    gems = produce(gems, draft => {
      allGems.forEach(() => draft.push(0));
    });

    treasures = produce(treasures, draft => {
      allTreasures.forEach(() => draft.push(0));
    });
	});
</script>

<svelte:window
  on:contextmenu={e => {
    if (import.meta.env.PROD) {
      e.preventDefault();
    }
  }}
  on:keydown={e => {
    if (e.key === 'F5') {
      e.preventDefault();
    }
  }}
/>

<main>
  <div class="gems">
    {#if allGems && gems}
      {#each allGems as { name, color, shape, value }, i}
        <NumericUpDown
          value={gems[i]}
          onChange={value => {
            gems = produce(gems, draft => {
              draft[i] = value;
            });
          }}
        >
          <Image
            src={`gems/${name.replaceAll(' ', '_').toLowerCase()}.png`}
            {name}
            tooltipText={`Shape: ${Shape[shape]}
              Color: ${Color[color]}
              Value: ${formatCurrency(value)}
            `}
          />
        </NumericUpDown>
      {/each}
    {/if}
  </div>

  <div class="treasures">
    {#if allTreasures && treasures}
      {#each allTreasures as { name, sockets, value }, i}
        <NumericUpDown
          value={treasures[i]}
          onChange={value => {
            treasures = produce(treasures, draft => {
              draft[i] = value;
            });
          }}
        >
          <Image
            src={`treasures/${name.replaceAll(' ', '_').toLowerCase()}.png`}
            {name}
            tooltipText={`Sockets: ${stringifySockets(sockets)}
              Value: ${formatCurrency(value)}
            `}
          />
        </NumericUpDown>
      {/each}
    {/if}
  </div>

  <div class="options">
    <div>
      <label for="allow-empty">Allow empty sockets</label>
      <input
        type="checkbox"
        id="allow-empty"
        bind:checked={allowEmptySockets}
      />
    </div>

    <div>
      <label for="objective">Objective Function</label>
      <select bind:value={selectedObjectiveFunction}>
        <option value={ObjectiveFunction.MaximizeBonus}>
          Maximize Bonus
        </option>
        <option value={ObjectiveFunction.MaximizeValue}>
          Maximize Value
        </option>
      </select>
    </div>
  </div>

  <button on:click={optimize}>
    Optimize
  </button>

  {#if total.value > 0}
    <div style="margin-top: 32px; margin-bottom: 16px;">
      Total value: {formatCurrency(total.value)} (+{total.bonus.toLocaleString('en')})
    </div>
  {/if}

  <div class="results">
    {#each filteredResults as { id, sockets, appraisal }}
      <div class="treasure">
        <div class="image">
          <Image
            src={`treasures/${getTreasure(id).name.replaceAll(' ', '_').toLowerCase()}.png`}
            name={getTreasure(id).name}
            tooltipText={`Gems: ${stringifyGems(sockets)}
              Value: ${formatCurrency(appraisal.value)} (+${appraisal.bonus.toLocaleString('en')})
            `}
            width={88}
            height={88}
          />
        </div>

        <div class="sockets">
          {#each sockets as { shape, gemId }}
            <SocketComponent
              {shape}
              color={getGem(gemId)?.color}
              height={shape === Shape.Circular ? 14 : 16}
            />
          {/each}
        </div>
      </div>
    {/each}
  </div>

  {#if !isNoneSelected}
    <button class="reset-button" on:click={resetSelection}>
      Reset
    </button>
  {/if}

</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 8px;
    text-align: center;
    user-select: none;
  }

  .gems {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 8px;
  }

  .treasures {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 8px;
  }

  .options {
    display: flex;
    gap: 32px;
    align-items: end;
    margin-bottom: 16px;
  }

  .results {
    display: flex;
    flex-wrap: wrap;
    column-gap: 8px;
    row-gap: 16px;
    margin-bottom: 16px;
  }

  .treasure .image {
    position: relative;
    border-radius: 8px;
    margin-bottom: 4px;
  }

  .sockets {
    display: grid;
    grid-auto-columns: min-content;
    grid-auto-flow: column;
    column-gap: 4px;
    justify-items: center;
    align-items: center;
    justify-content: center;
  }

  .reset-button {
    position: fixed;
    z-index: 9;
    bottom: 16px;
    right: 16px;
  }
</style>
