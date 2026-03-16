<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  
  // Svelte 5のRunesを使用した状態管理
  let theme = $state("");
  let gameState = $state(null);
  let loading = $state(false);
  let errorMessage = $state("");

  /**
   * 新しいゲームを開始し、バックエンドから初期状態を取得する
   */
  async function startNewGame() {
    errorMessage = "";
    loading = true;
    try {
      // Tauriコマンド 'start_game' を呼び出し
      gameState = await invoke("start_game", { theme: theme || null });
    } catch (e) {
      console.error("ゲーム開始エラー:", e);
      errorMessage = "ゲームを開始できませんでした。時間をおいて再試行してください。";
    } finally {
      loading = false;
    }
  }
</script>

<main class="landing-shell container mx-auto flex min-h-screen max-w-5xl flex-col justify-center px-4 py-5 sm:px-5 sm:py-6 md:p-8">
  <div aria-hidden="true" class="orb orb-one"></div>
  <div aria-hidden="true" class="orb orb-two"></div>
  <div aria-hidden="true" class="grid-dust"></div>
  {#if !gameState}
    <!-- バトル開始前の初期画面 -->
    <div class="intro-panel grid gap-5 md:gap-6 lg:grid-cols-[1.02fr_0.98fr] lg:items-center lg:gap-10">
      <section class="hero-copy text-center lg:text-left">
        <p class="hero-badge">Desktop Debate Arena</p>
        <h1 class="hero-title py-2" style="text-wrap: balance;">
          DEBATTLERS
        </h1>
        <p class="hero-lead">
          論理とカードで世界を説得する。明るく遊べて、しっかり考える。小学生から高校生まで入りやすい討論バトルです。
        </p>

        <div class="hero-pills" aria-hidden="true">
          <span class="hero-pill">即興テーマ</span>
          <span class="hero-pill">AI対戦</span>
          <span class="hero-pill">ワンボタン開始</span>
        </div>
      </section>

      <section class="play-card glass-panel space-y-4 p-5 text-left sm:p-6 md:space-y-5 md:p-7">
        <div class="flex items-start justify-between gap-3 sm:gap-4">
          <div>
            <p class="text-xs font-semibold uppercase tracking-[0.24em] text-sky-700">Quick Match</p>
            <h2 class="mt-1.5 text-[1.65rem] font-black text-slate-800 sm:mt-2 sm:text-2xl">討論を始める</h2>
          </div>
          <div class="status-chip">1 tap</div>
        </div>

        <label class="space-y-2" for="theme-input">
          <span class="block text-sm font-medium uppercase tracking-[0.18em] text-slate-700">討論テーマ</span>
          <input 
            id="theme-input"
            name="theme"
            bind:value={theme}
            autocomplete="off"
            placeholder="例: AIは教育を変えるべきか…"
            class="w-full rounded-[1.2rem] border border-sky-200 bg-white/90 px-4 py-3.5 text-left text-slate-800 placeholder:text-slate-400 transition-[border-color,box-shadow,background-color,transform] focus-visible:border-cyan-500 focus-visible:outline-2 focus-visible:outline-cyan-500 focus-visible:outline-offset-2 sm:px-5 sm:py-4"
          />
        </label>
        <p class="text-sm text-slate-600">空欄ならランダムテーマでそのまま開始します。</p>
        {#if errorMessage}
          <p aria-live="polite" class="rounded-2xl border border-rose-300 bg-rose-50 px-4 py-3 text-sm text-rose-700">
            {errorMessage}
          </p>
        {/if}
        <button 
          onclick={startNewGame}
          disabled={loading}
          class="rounded-[1.2rem] bg-gradient-to-r from-amber-400 via-orange-400 to-pink-400 p-3.5 text-lg font-black text-slate-900 shadow-[0_16px_40px_rgba(251,191,36,0.28)] transition-[transform,opacity,box-shadow,filter] hover:brightness-105 focus-visible:outline-2 focus-visible:outline-orange-400 focus-visible:outline-offset-2 active:scale-[0.98] disabled:cursor-not-allowed disabled:opacity-70 sm:p-4"
        >
          {loading ? "準備中…" : "バトル開始"}
        </button>

        <div class="grid grid-cols-3 gap-2 pt-1 text-center sm:gap-3 sm:pt-2" aria-hidden="true">
          <div class="mini-stat">
            <strong>3</strong>
            <span>ターン</span>
          </div>
          <div class="mini-stat">
            <strong>AI</strong>
            <span>対戦相手</span>
          </div>
          <div class="mini-stat">
            <strong>∞</strong>
            <span>テーマ</span>
          </div>
        </div>
      </section>
    </div>
  {:else}
    <!-- バトル進行中の画面 -->
    <div class="space-y-6">
      <header class="glass-panel relative flex items-start justify-between gap-4 overflow-hidden p-5 sm:p-6">
        <div aria-hidden="true" class="header-glow"></div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold uppercase tracking-[0.24em] text-sky-700">Live Debate</p>
          <h2 class="mt-2 break-words text-2xl font-black text-slate-800">{gameState.theme}</h2>
          <p class="mt-2 text-sm text-slate-600">ターン {gameState.turn_count}</p>
        </div>
        <div class="shrink-0 text-right">
          <p class="text-sm font-semibold uppercase tracking-[0.2em] text-pink-600">Your Side</p>
          <p class="mt-2 font-bold text-sky-700">{gameState.player_position}</p>
          <p class="text-xs text-rose-600">vs {gameState.opponent_position}</p>
        </div>
      </header>

      <!-- TODO: ゲージ、手札、履歴などのコンポーネントをここに実装予定 -->
      <div class="glass-panel p-10 text-center italic text-slate-600 sm:p-16 md:p-20">
        バトル画面を実装中… (WSL環境への移行をお待ちください)
      </div>
    </div>
  {/if}
</main>

<style>
  /* Svelte 固有のアニメーション等はここに記述 */
</style>
