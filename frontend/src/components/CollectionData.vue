<script lang="ts">
    import { defineComponent } from 'vue';
    import type {Collection} from '../models/collection.model';
    import { CollectionService } from '../service/collection.service';

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"collection-data",
        data() {
            return {
                selectedCollection: "",
                collection: {} as Collection
            }
        },
        methods: {
            displayCollection(collectionId : string) : void {
                console.log("displaying collections from id : " + collectionId);
                collectionService.getCollection(collectionId).then(collectionRes => {
                    this.collection = collectionRes;
                });
            }
        }
    });
</script>

<template>
    <div>
        <h2><a v-bind:href="collection.url">{{ collection.name }}</a></h2>
        <ui-grid>
            <ui-grid-cell></ui-grid-cell>
            <ui-grid-cell>
                <ui-list>
                    <ui-item v-for="childCol in collection.children_col" :key="childCol">
                        <ui-item-text-content>
                            <ui-item-text1>
                                <a v-bind:href="childCol.url">{{ childCol.name }}</a>
                            </ui-item-text1>
                        </ui-item-text-content>
                        <ui-item-last-content>
                            <ui-icon>queue_music</ui-icon>
                        </ui-item-last-content>
                    </ui-item>
                </ui-list>
                <ui-list>
                    <ui-item v-for="track in collection.tracks" :key="track">
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