<script lang="ts">
    import { defineComponent } from 'vue';
    import type {Track} from '../models/collection.model';
    import { CollectionService } from '../service/collection.service';

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"collection-data",
        data() {
            return {
                selectedCollection: "",
                collectionName: "",
                tracks: [] as Track[]
            }
        },
        methods: {
            displayCollection(collectionId : string) : void {
                console.log("displaying collections from id : " + collectionId);
                collectionService.getCollection(collectionId).then(collection => {
                    this.tracks = collection.tracks;
                });
            }
        }
    });
</script>

<template>
    <div>
        <h2>Collection data</h2>
        <ui-grid>
            <ui-grid-cell></ui-grid-cell>
            <ui-grid-cell>
                <ui-list>
                    <ui-item v-for="track in tracks" :key="track">
                        <ui-item-text-content>
                            <ui-item-text1>
                                <a v-bind:href="track.link">{{ track.title }}</a>
                            </ui-item-text1>
                            <ui-item-text2>
                                {{ track.artist }}
                            </ui-item-text2>
                        </ui-item-text-content>
                        <ui-item-last-content>
                            <ui-icon>music_note</ui-icon>
                        </ui-item-last-content>
                    </ui-item>
                </ui-list>
            </ui-grid-cell>
            <ui-grid-cell></ui-grid-cell>
        </ui-grid>
    </div>
</template>