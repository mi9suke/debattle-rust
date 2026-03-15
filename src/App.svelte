<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  
  // Svelte 5のRunesを使用した状態管理
  let theme = $state("");
  let gameState = $state(null);
  let loading = $state(false);

  /**
   * 新しいゲームを開始し、バックエンドから初期状態を取得する
   */
  async function startNewGame() {
    loading = true;
    try {
      // Tauriコマンド 'start_game' を呼び出し
      gameState = await invoke("start_game", { theme: theme || null });
    } catch (e) {
      console.error("ゲーム開始エラー:", e);
    } finally {
      loading = false;
    }
  }
</script>

<main class="container mx-auto p-8 max-w-4xl h-screen flex flex-col justify-center">
  {#if !gameState}
    <!-- バトル開始前の初期画面 -->
    <div class="glass-panel p-12 text-center space-y-8 animate-in fade-in zoom-in duration-500">
      <h1 class="text-6xl font-black tracking-tight gradient-text py-2">
        DEBATTLERS
      </h1>
      <p class="text-slate-400 text-lg">
        論理とカードで世界を説得する。2026年最新の討論バトル。
      </p>
      
      <div class="flex flex-col space-y-4 max-w-md mx-auto pt-8">
        <input 
          bind:value={theme}
          placeholder="討論テーマを入力（空欄でランダム）"
          class="bg-slate-900 border border-slate-800 rounded-xl p-4 text-center focus:ring-2 focus:ring-cyan-500 outline-none transition-all"
        />
        <button 
          onclick={startNewGame}
          disabled={loading}
          class="bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 p-4 rounded-xl font-bold text-lg shadow-lg shadow-cyan-900/20 transform active:scale-95 transition-all disabled:opacity-50"
        >
          {loading ? "準備中..." : "バトル開始"}
        </button>
      </div>
    </div>
  {:else}
    <!-- バトル進行中の画面 -->
    <div class="space-y-6">
      <header class="flex justify-between items-center glass-panel p-6">
        <div>
          <h2 class="text-2xl font-bold">{gameState.theme}</h2>
          <p class="text-sm text-slate-400">ターン {gameState.turn_count}</p>
        </div>
        <div class="text-right">
          <p class="text-cyan-400 font-bold">{gameState.player_position}</p>
          <p class="text-xs text-rose-400">vs {gameState.opponent_position}</p>
        </div>
      </header>

      <!-- TODO: ゲージ、手札、履歴などのコンポーネントをここに実装予定 -->
      <div class="text-center p-20 text-slate-500 italic">
        バトル画面を実装中... (WSL環境への移行をお待ちください)
      </div>
    </div>
  {/if}
</main>

<style>
  /* Svelte 固有のアニメーション等はここに記述 */
</style>
