<script lang="ts">
import { defineComponent } from 'vue';
import { CollectionService } from '../service/collection.service';
import type Collection from '../models/collection.model';

    type SelectOption = {
        label : string,
        value : string,
        disabled ?: boolean
    }
    let collectionService = new CollectionService();

    export default defineComponent({
        name:"collection-select",
        data() {
            return {
                options: [] as SelectOption[],
                selectedOption: {} as SelectOption,
                open: false,
                collectionName: "",
                fromPlaylistChecked: false,
                fromPlaylist: ""
            }
        },
        methods: {
            retrieveCollections() {
                collectionService
                    .listCollections()
                    .then((collections) => {
                        this.options = collections.map((collection) => this.collectionToSelectOption(collection));
                        this.selectedOption = this.options[0]
                        console.log("options = " + this.options);
                    })
            },
            collectionToSelectOption(collection : Collection) : SelectOption {
                return {
                    label:collection.name,
                    value:collection.name
                };
            },
            onSelectCollection(pSelectedOption : any) {
                this.selectedOption = pSelectedOption;
                console.log(this.selectedOption.value);
            },
            addCollection() {
                this.open = true;
            },
            onConfirm() {
                console.log("add collection " + this.collectionName + " from playlist " + this.fromPlaylist);
                collectionService.initCollection(this.collectionName, this.fromPlaylist);
            }
        },
        mounted() {
            this.retrieveCollections();
        }
    });
</script>

<template>
    <div>
    <center>
        <v-container>
            <ui-select v-model="selectedOption" :options="options" @selected="onSelectCollection($event)"></ui-select>
            <ui-icon-button icon="add" @click="addCollection()"></ui-icon-button>
        </v-container>
    </center>
    <ui-dialog v-model="open" @confirm="onConfirm">
        <ui-dialog-title>Enter the collection name</ui-dialog-title>
        <ui-dialog-content>
            <form method="dialog">
                <div>
                    <label>Collection name :</label>
                    <ui-textfield
                        v-model="collectionName"
                        helper-text-id="collection-name-field-helper-text"
                    >
                        Collection name
                    </ui-textfield>
                </div>
                <div>
                    <ui-form-field>
                        <label>From playlist :</label>
                        <ui-checkbox v-model="fromPlaylistChecked" input-id="checkbox"></ui-checkbox>
                    </ui-form-field>
                </div>
                <div v-if="fromPlaylistChecked">
                    <label>From playlist :</label>
                    <ui-textfield
                        v-model="fromPlaylist"
                        helper-text-id="from-playlist-field-helper-text"
                    >
                        Deezer playlist URL
                    </ui-textfield>
                </div>
            </form>
        </ui-dialog-content>
        <ui-dialog-actions></ui-dialog-actions>
    </ui-dialog>
    </div>
</template>
