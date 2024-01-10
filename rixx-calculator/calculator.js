const mapLogarithmic = (value, inMin, inMax, outMin, outMax) => {
  const inRange = inMax - inMin;
  const outRange = Math.log(outMax / outMin);
  const scale = outRange / inRange;

  return Math.exp(scale * (value - inMin)) * outMin;
};

const sum = (arr) => arr.reduce((a, b) => a + b, 0);
const max = (arr) => Math.max(...arr);

const Rune = {
  props: ["rune", "equipmentdraggable", "othersdraggable"],
  computed: {
    draggable() {
      if (
        this.rune.tags.includes("other_item") ||
        (this.equipmentdraggable && this.rune.tags.includes("equipment")) ||
        (this.othersdraggable && !this.rune.tags.includes("equipment"))
      ) {
        return "draggable";
      }
    },
  },
  template: `
    <div class="box rune" :class="draggable">
      <div class="pure-g">
        <div class="pure-u-1-3">
          <div class="left">
            <span class="index">{{ rune.index }} #&nbsp;&nbsp;</span><br /><br />
            <span class="zincost">{{ rune.zinium }} zin</span><br />
            <span class="slots">
              {{ rune.slots > 0 ? "+" : "" }}{{ rune.slots }} slo </span><br />
            <span class="blueprint" v-if="rune.blueprint">
              {{ rune.blueprint }} bp&nbsp;
            </span>
          </div>
        </div>
        <div class="pure-u-2-3">
          <div class="right">
            <span class="shortname">{{ rune.shortname }}</span
            ><br /><br />
            <span class="longname">{{ rune.longname }}</span><br />
          </div>
        </div>
      </div>
    </div>
        `,
};

const app = Vue.createApp({
  data() {
    return {
      checked: {
        equipment: true,
        stat: true,
        bonus: true,
        hpspepmax: true,
        spellskill: true,
        weight: true,
        exprate: true,
        haste: true,
        special: true,
        slots0: true,
        slots1: true,
        slots2: true,
        slots4: true,
      },
      runes: [],
      zinpct: 44,
      showfilters: true,
      textfilter: "",
      rixxeq1: [],
      rixxeq2: [],
      rixxeq3: [],
      othereq: [],
    };
  },
  created() {
    fetch("data.json")
      .then((response) => {
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        return response.json();
      })
      .then((data) => {
        this.runes = data;
        this.parsequeryparams();
      })
      .catch((error) => {
        console.error("There was a problem loading the data:", error);
      });
  },
  components: {
    draggable: VueDraggablePlus.VueDraggable,
    Rune,
  },
  computed: {
    filteredRunes() {
      return this.runes
        .filter(
          (rune) =>
            // rune is in equipment
            !this.rixxeq1ids.includes(rune.index) &&
            !this.rixxeq2ids.includes(rune.index) &&
            !this.rixxeq3ids.includes(rune.index) &&
            !this.othereqids.includes(rune.index) &&
            // text
            (rune.shortname
              .toLowerCase()
              .includes(this.textfilter.toLowerCase()) ||
              rune.longname
                .toLowerCase()
                .includes(this.textfilter.toLowerCase()) ||
              (rune.blueprint &&
                rune.blueprint
                  .toLowerCase()
                  .includes(this.textfilter.toLowerCase()))) &&
            // zin
            rune.zinium <= this.zinabs &&
            // type
            ((this.checked.equipment && rune.tags.includes("equipment")) ||
              (this.checked.stat && rune.tags.includes("stat")) ||
              (this.checked.bonus && rune.tags.includes("bonus")) ||
              (this.checked.hpspepmax && rune.tags.includes("hpspepmax")) ||
              (this.checked.spellskill && rune.tags.includes("spellskill")) ||
              (this.checked.weight && rune.tags.includes("weight")) ||
              (this.checked.exprate && rune.tags.includes("exprate")) ||
              (this.checked.haste && rune.tags.includes("haste")) ||
              (this.checked.special && rune.tags.includes("special"))) &&
            // slots
            ((this.checked.slots0 && rune.slots >= 0) ||
              (this.checked.slots1 && rune.slots === -1) ||
              (this.checked.slots2 && rune.slots === -2) ||
              (this.checked.slots4 && rune.slots <= -4))
        )
        .sort((a, b) => a.index - b.index);
    },
    filteredhasequipment() {
      return (
        this.filteredRunes.filter((rune) => rune.tags.includes("equipment"))
          .length > 0
      );
    },
    typesselected() {
      return (
        this.checked.equipment &&
        this.checked.stat &&
        this.checked.bonus &&
        this.checked.hpspepmax &&
        this.checked.spellskill &&
        this.checked.weight &&
        this.checked.exprate &&
        this.checked.haste &&
        this.checked.special
      );
    },
    zinabs() {
      return Math.round(mapLogarithmic(this.zinpct, 0, 100, 50, 50000));
    },
    rixxeq1ids() {
      return this.rixxeq1.map((rune) => rune.index);
    },
    rixxeq2ids() {
      return this.rixxeq2.map((rune) => rune.index);
    },
    rixxeq3ids() {
      return this.rixxeq3.map((rune) => rune.index);
    },
    othereqids() {
      return this.othereq.map((rune) => rune.index);
    },
    rixxeq1slots() {
      return sum(this.rixxeq1.map((rune) => rune.slots));
    },
    rixxeq2slots() {
      return sum(this.rixxeq2.map((rune) => rune.slots));
    },
    rixxeq3slots() {
      return sum(this.rixxeq3.map((rune) => rune.slots));
    },
    rixxeq2shown() {
      return (
        this.rixxeq2.length > 0 ||
        (this.rixxeq1.length > 0 && this.filteredhasequipment)
      );
    },
    rixxeq3shown() {
      return (
        this.rixxeq3.length > 0 ||
        (this.rixxeq2.length > 0 && this.filteredhasequipment)
      );
    },
    othereqshown() {
      return (
        this.othereqids > 0 ||
        this.filteredRunes.filter((rune) => rune.tags.includes("other_item"))
          .length > 0
      );
    },
    maxzin() {
      const runes = this.rixxeq1.concat(this.rixxeq2).concat(this.rixxeq3);
      return max(runes.map((rune) => rune.zinium));
    },
    queryparams() {
      const url = new URL(window.location.href);
      if (this.rixxeq1ids.length > 0) {
        url.searchParams.set("eq1", this.rixxeq1ids.join(","));
      } else {
        url.searchParams.delete("eq1");
      }
      if (this.rixxeq2ids.length > 0) {
        url.searchParams.set("eq2", this.rixxeq2ids.join(","));
      } else {
        url.searchParams.delete("eq2");
      }
      if (this.rixxeq3ids.length > 0) {
        url.searchParams.set("eq3", this.rixxeq3ids.join(","));
      } else {
        url.searchParams.delete("eq3");
      }
      if (this.othereqids.length > 0) {
        url.searchParams.set("othereq", this.othereqids.join(","));
      } else {
        url.searchParams.delete("othereq");
      }
      return url.searchParams.toString();
    },
  },
  methods: {
    toggletypes(event) {
      if (this.typesselected) {
        this.checked.equipment = false;
        this.checked.stat = false;
        this.checked.bonus = false;
        this.checked.hpspepmax = false;
        this.checked.spellskill = false;
        this.checked.weight = false;
        this.checked.exprate = false;
        this.checked.haste = false;
        this.checked.special = false;
      } else {
        this.checked.equipment = true;
        this.checked.stat = true;
        this.checked.bonus = true;
        this.checked.hpspepmax = true;
        this.checked.spellskill = true;
        this.checked.weight = true;
        this.checked.exprate = true;
        this.checked.haste = true;
        this.checked.special = true;
      }
    },
    parsequeryparams() {
      const url = new URL(window.location.href);
      const eq1 = url.searchParams.get("eq1");
      const eq2 = url.searchParams.get("eq2");
      const eq3 = url.searchParams.get("eq3");
      const othereq = url.searchParams.get("othereq");
      if (eq1) {
        this.rixxeq1 = eq1.split(",").map((id) => {
          return this.runes.find((rune) => rune.index === parseInt(id));
        });
      }
      if (eq2) {
        this.rixxeq2 = eq2.split(",").map((id) => {
          return this.runes.find((rune) => rune.index === parseInt(id));
        });
      }
      if (eq3) {
        this.rixxeq3 = eq3.split(",").map((id) => {
          return this.runes.find((rune) => rune.index === parseInt(id));
        });
      }
      if (othereq) {
        this.othereq = othereq.split(",").map((id) => {
          return this.runes.find((rune) => rune.index === parseInt(id));
        });
      }
    },
    rixxeqchanged() {
      this.rixxeqrules(this.rixxeq1);
      console.log("rixxeqchanged");
    },
    rixxeqrules(runes) {
      console.log(runes);
      // if eq contains only one item, it must be equipment
      if (runes.length === 1) {
        if (!runes[0].tags.includes("equipment")) {
          console.log("First rune must be equipment");
          runes.pop();
        }
      }
    },
  },
  watch: {
    queryparams() {
      window.history.replaceState(
        null,
        null,
        window.location.pathname + "?" + this.queryparams
      );
    },
  },
});

app.mount("#app");
