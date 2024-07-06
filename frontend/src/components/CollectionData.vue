<script lang="ts">
    import { defineComponent } from 'vue';
    import type {Collection, Track} from '../models/collection.model';
    import { CollectionService } from '../service/collection.service';
    import { type SelectOption } from "../models/balmui.model"
    import AddChildCollectionDialog from './dialogs/AddChildCollectionDialog.vue';
    import RemoveChildCollectionDialog from './dialogs/RemoveChildCollectionDialog.vue';
    import RemoveCollectionDialog from './dialogs/RemoveCollectionDialog.vue';

    let collectionService = new CollectionService();
    

    export default defineComponent({
        name:"collection-data",
        components: {
            AddChildCollectionDialog,
            RemoveCollectionDialog,
            RemoveChildCollectionDialog,
        },
        data() {
            return {
                collection: {} as Collection,
                childrenCollections: [] as Collection[],
                tracks: [] as Track[],
                collections: [] as SelectOption[],
            }
        },
        methods: {
            displayCollection(collectionId : string) : void {
                console.log("displaying collections from id : " + collectionId);
                collectionService.getCollection(collectionId).then(collectionRes => {
                    this.collection = collectionRes;
                });
                collectionService.getChildrenCollections(collectionId).then(childrenCol => {
                    this.childrenCollections = childrenCol;
                });
                collectionService.getCollectionTracks(collectionId).then(tracksRes => {
                    this.tracks = tracksRes;
                });
            },
            updateCollection() : void {
                collectionService.updateCollection(this.collection.deezer_id);
            },
            addChildCollection() : void {
                this.$refs.addChildCollectionDialogRef.openDialog();
            },
            openRemoveChildCollection(childCollection : Collection) {
                this.$refs.removeChildCollectionDialogRef.setChildCollection(childCollection);
                this.$refs.removeChildCollectionDialogRef.openDialog();
            },
            openRemoveCollection() {
                this.$refs.removeCollectionDialogRef.openDialog();
            }
        }
    });
</script>

<template>
    <div>
        <h2>
            <v-container>
                <a v-bind:href="collection.url">{{ collection.name }}</a>
                <ui-icon-button icon="add" @click="addChildCollection()"></ui-icon-button>
                <ui-icon-button icon="clear" @click="openRemoveCollection()"></ui-icon-button>
            </v-container>
        </h2>
        <ui-button @click="updateCollection">Update collection</ui-button>
        <ui-grid>
            <ui-grid-cell></ui-grid-cell>
            <ui-grid-cell>
                <ui-list>
                    <ui-item v-for="childCol in childrenCollections" :key="childCol">
                        <ui-item-first-content>
                            <ui-icon>queue_music</ui-icon>
                        </ui-item-first-content>
                        <ui-item-text-content>
                            <ui-item-text1>
                                <a v-bind:href="childCol.url">{{ childCol.name }}</a>
                            </ui-item-text1>
                        </ui-item-text-content>
                        <ui-item-last-content>
                            <ui-icon-button icon="clear" @click="openRemoveChildCollection(childCol)"></ui-icon-button>
                        </ui-item-last-content>
                    </ui-item>
                </ui-list>
                <ui-list>
                    <ui-item v-for="track in tracks" :key="track">
                        <ui-item-first-content>
                            <ui-icon>music_note</ui-icon>
                        </ui-item-first-content>
                        <ui-item-text-content>
                            <ui-item-text1>
                                <a v-bind:href="track.link">{{ track.title }}</a>
                            </ui-item-text1>
                            <ui-item-text2>
                                {{ track.artist }}
                            </ui-item-text2>
                        </ui-item-text-content>
                    </ui-item>
                </ui-list>
            </ui-grid-cell>
            <ui-grid-cell></ui-grid-cell>
        </ui-grid>
        <AddChildCollectionDialog ref="addChildCollectionDialogRef" v-bind:collections="collections" v-bind:collection="collection" />
        <RemoveChildCollectionDialog ref="removeChildCollectionDialogRef" v-bind:collection="collection" />
        <RemoveCollectionDialog ref="removeCollectionDialogRef" v-bind:collection="collection" />
    </div>
</template>